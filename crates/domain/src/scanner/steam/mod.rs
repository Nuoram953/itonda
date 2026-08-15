use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use tracing::{debug, info};

use crate::{
    media::types::{MediaLaunchType, MediaSource, MediaType},
    scanner::{
        errors::ScannerError,
        models::{ScannedLaunch, ScannedMedia},
        steam::{parser::parse_library_folders, paths::find_steam_install_dirs},
        traits::MediaScanner,
    },
};

pub mod parser;
pub mod paths;

#[cfg(test)]
mod tests;

pub struct SteamScanner {
    custom_paths: Vec<PathBuf>,
}

impl Default for SteamScanner {
    fn default() -> Self {
        Self::new()
    }
}

impl SteamScanner {
    pub fn new() -> Self {
        Self {
            custom_paths: Vec::new(),
        }
    }

    pub fn with_paths(custom_paths: Vec<PathBuf>) -> Self {
        Self { custom_paths }
    }

    pub fn discover_library_folders(&self) -> Vec<PathBuf> {
        let mut library_paths = Vec::new();

        let steam_roots = if self.custom_paths.is_empty() {
            find_steam_install_dirs()
        } else {
            self.custom_paths.clone()
        };

        for root in steam_roots {
            if root.exists() && !library_paths.contains(&root) {
                library_paths.push(root.clone());
            }

            let vdf_path = root.join("steamapps").join("libraryfolders.vdf");
            if vdf_path.exists()
                && let Ok(content) = std::fs::read_to_string(&vdf_path)
                && let Ok(folders) = parse_library_folders(&content)
            {
                for folder in folders {
                    if folder.path.exists() && !library_paths.contains(&folder.path) {
                        library_paths.push(folder.path);
                    }
                }
            }
        }

        library_paths
    }

    pub fn scan_library_folder(&self, library_dir: &Path) -> Vec<ScannedMedia> {
        let steamapps_dir = library_dir.join("steamapps");
        let common_dir = steamapps_dir.join("common");
        let mut items = Vec::new();

        let Ok(entries) = std::fs::read_dir(&steamapps_dir) else {
            return items;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            if file_name.starts_with("appmanifest_") && file_name.ends_with(".acf") {
                let Ok(content) = std::fs::read_to_string(&path) else {
                    continue;
                };

                let Ok(manifest) = parser::parse_app_manifest(&content) else {
                    continue;
                };

                // if STEAM_TOOL_APP_IDS.contains(&manifest.app_id.as_str()) {
                //     continue;
                // }

                if !manifest.is_fully_installed {
                    continue;
                }

                let working_dir = common_dir.join(&manifest.install_dir);

                items.push(ScannedMedia {
                    media_type: MediaType::Game,
                    title: manifest.name,
                    external_id: Some(manifest.app_id.clone()),
                    source: MediaSource::Steam.as_str().into(),
                    working_directory: Some(working_dir.to_string_lossy().to_string()),
                    launch: Some(ScannedLaunch {
                        name: "Steam".into(),
                        launch_type: MediaLaunchType::Storefront,
                        program: "steam".into(),
                        arguments: vec![format!("steam://run/{}", manifest.app_id)],
                        working_directory: Some(working_dir.to_string_lossy().to_string()),
                    }),
                });
            }
        }

        items
    }
}

#[async_trait]
impl MediaScanner for SteamScanner {
    fn supported_media_type(&self) -> Vec<MediaType> {
        vec![MediaType::Game]
    }
    fn name(&self) -> &'static str {
        "Steam"
    }
    fn is_available(&self) -> bool {
        true
    }
    async fn scan(&self) -> Result<Vec<ScannedMedia>, ScannerError> {
        let libraries = self.discover_library_folders();

        debug!(
            "Discovered {} Steam library folders to scan",
            libraries.len()
        );

        let mut all_games = Vec::new();
        let mut seen_app_ids = HashSet::new();

        for library_path in libraries {
            debug!("Scanning Steam library: {:?}", library_path);
            let games = self.scan_library_folder(&library_path);

            for game in games {
                if let Some(app_id) = &game.external_id
                    && seen_app_ids.insert(app_id.clone())
                {
                    all_games.push(game);
                }
            }
        }
        info!(
            "Steam scan complete: found {} installed games",
            all_games.len()
        );

        Ok(all_games)
    }
}

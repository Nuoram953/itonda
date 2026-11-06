use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::process::Command;

use crate::{
    assets::types::AssetType,
    sources::youtube::models::{YoutubeVideoInfo, YtDlpRawOutput},
};

#[derive(Debug, Error)]
pub enum YoutubeError {
    #[error("I/O error executing yt-dlp: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse yt-dlp JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("yt-dlp command failed with code {code:?}: {stderr}")]
    CommandFailed {
        code: Option<i32>,
        stderr: String,
    },

    #[error("No video info found for query or ID: {0}")]
    NotFound(String),
}

#[derive(Clone, Debug, Default)]
pub struct YoutubeClient {
    binary_path: String,
}

impl YoutubeClient {
    pub fn expand_path(p: &str) -> String {
        let trimmed = p.trim();
        if trimmed.starts_with("~/") {
            if let Some(home) = dirs::home_dir() {
                return home.join(&trimmed[2..]).to_string_lossy().to_string();
            }
        }
        if trimmed == "Downloads/yt-dlp" {
            if let Some(home) = dirs::home_dir() {
                let candidate = home.join("Downloads/yt-dlp");
                if candidate.exists() {
                    return candidate.to_string_lossy().to_string();
                }
            }
        }
        trimmed.to_string()
    }

    pub fn resolve_binary_path(custom: Option<&str>) -> String {
        if let Some(custom) = custom {
            let expanded = Self::expand_path(custom);
            if !expanded.trim().is_empty() {
                return expanded;
            }
        }

        if let Ok(env_path) = std::env::var("ITONDA_YTDLP_PATH").or_else(|_| std::env::var("YTDLP_PATH")) {
            let expanded = Self::expand_path(&env_path);
            if !expanded.trim().is_empty() {
                return expanded;
            }
        }

        if let Some(home) = dirs::home_dir() {
            let user_downloads = home.join("Downloads/yt-dlp");
            if user_downloads.exists() {
                return user_downloads.to_string_lossy().to_string();
            }
        }

        let local_downloads = std::path::Path::new("Downloads/yt-dlp");
        if local_downloads.exists() {
            return local_downloads.to_string_lossy().to_string();
        }

        "yt-dlp".to_string()
    }

    pub fn new() -> Self {
        Self {
            binary_path: Self::resolve_binary_path(None),
        }
    }

    pub fn with_binary_path(binary_path: impl Into<String>) -> Self {
        let path = binary_path.into();
        Self {
            binary_path: Self::resolve_binary_path(Some(&path)),
        }
    }

    pub fn binary_path(&self) -> &str {
        &self.binary_path
    }

    pub fn extract_video_id(input: &str) -> Option<String> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return None;
        }

        if let Ok(url) = url::Url::parse(trimmed) {
            if let Some(host) = url.host_str() {
                if host.contains("youtube.com") {
                    if let Some(pair) = url.query_pairs().find(|(k, _)| k == "v") {
                        return Some(pair.1.to_string());
                    }
                    if url.path().starts_with("/shorts/") {
                        let id = url.path().trim_start_matches("/shorts/").split('/').next()?;
                        if !id.is_empty() {
                            return Some(id.to_string());
                        }
                    }
                } else if host == "youtu.be" {
                    let id = url.path().trim_start_matches('/').split('/').next()?;
                    if !id.is_empty() {
                        return Some(id.to_string());
                    }
                }
            }
        }

        // Check if alphanumeric ID of standard length (usually 11 chars)
        if trimmed.len() == 11 && trimmed.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Some(trimmed.to_string());
        }

        None
    }

    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<YoutubeVideoInfo>, YoutubeError> {
        self.search_with_filter(query, limit, None).await
    }

    pub async fn search_with_filter(
        &self,
        query: &str,
        limit: usize,
        max_duration_secs: Option<u64>,
    ) -> Result<Vec<YoutubeVideoInfo>, YoutubeError> {
        let trimmed = query.trim();
        if trimmed.is_empty() {
            return Ok(Vec::new());
        }

        // If it's a direct URL or ID, fetch single video
        if let Some(id) = Self::extract_video_id(trimmed) {
            match self.get_video_info(&id).await {
                Ok(info) => {
                    if let Some(max_secs) = max_duration_secs {
                        if info.duration.map_or(false, |d| d <= max_secs) {
                            return Ok(vec![info]);
                        } else {
                            return Ok(Vec::new());
                        }
                    }
                    return Ok(vec![info]);
                }
                Err(err) => tracing::warn!("Failed to fetch direct video info for ID {id}: {err}"),
            }
        }

        let search_target = format!("ytsearch{limit}:{trimmed}");
        let mut cmd = Command::new(&self.binary_path);
        cmd.arg("--dump-single-json")
            .arg("--no-playlist")
            .arg("--skip-download");

        if let Some(max_secs) = max_duration_secs {
            cmd.arg("--match-filter")
                .arg(format!("duration <= {max_secs}"));
        }

        cmd.arg(&search_target);
        let output = cmd.output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            if stderr.contains("does not pass filter") || stderr.contains("No video formats found") {
                return Ok(Vec::new());
            }
            return Err(YoutubeError::CommandFailed {
                code: output.status.code(),
                stderr,
            });
        }

        let parsed: YtDlpRawOutput = serde_json::from_slice(&output.stdout)?;
        if let Some(entries) = parsed.entries {
            let results: Vec<YoutubeVideoInfo> = entries
                .into_iter()
                .flatten()
                .filter_map(|entry| entry.into_video_info())
                .filter(|info| {
                    if let Some(max_secs) = max_duration_secs {
                        info.duration.map_or(false, |d| d <= max_secs)
                    } else {
                        true
                    }
                })
                .collect();
            Ok(results)
        } else if let Some(single) = parsed.into_video_info() {
            if let Some(max_secs) = max_duration_secs {
                if single.duration.map_or(false, |d| d <= max_secs) {
                    Ok(vec![single])
                } else {
                    Ok(Vec::new())
                }
            } else {
                Ok(vec![single])
            }
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn get_video_info(&self, id_or_url: &str) -> Result<YoutubeVideoInfo, YoutubeError> {
        let target = if let Some(id) = Self::extract_video_id(id_or_url) {
            format!("https://www.youtube.com/watch?v={id}")
        } else {
            id_or_url.to_string()
        };

        let output = Command::new(&self.binary_path)
            .arg("--dump-single-json")
            .arg("--no-playlist")
            .arg("--skip-download")
            .arg(&target)
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(YoutubeError::CommandFailed {
                code: output.status.code(),
                stderr,
            });
        }

        let parsed: YtDlpRawOutput = serde_json::from_slice(&output.stdout)?;
        parsed
            .into_video_info()
            .ok_or_else(|| YoutubeError::NotFound(id_or_url.to_string()))
    }

    pub async fn download(
        &self,
        id_or_url: &str,
        asset_type: AssetType,
        output_path: &Path,
    ) -> Result<PathBuf, YoutubeError> {
        let target = if let Some(id) = Self::extract_video_id(id_or_url) {
            format!("https://www.youtube.com/watch?v={id}")
        } else {
            id_or_url.to_string()
        };

        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let mut cmd = Command::new(&self.binary_path);
        cmd.arg("--no-playlist")
            .arg("-o")
            .arg(output_path);

        match asset_type {
            AssetType::Audio => {
                cmd.arg("-x")
                    .arg("--audio-format")
                    .arg("mp3")
                    .arg("--audio-quality")
                    .arg("0")
                    .arg("--match-filter")
                    .arg("duration <= 300");
            }
            _ => {
                cmd.arg("-f")
                    .arg("bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best");
            }
        }

        cmd.arg(&target);

        let output = cmd.output().await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(YoutubeError::CommandFailed {
                code: output.status.code(),
                stderr,
            });
        }

        if !output_path.exists() {
            return Err(YoutubeError::CommandFailed {
                code: output.status.code(),
                stderr: "Downloaded file was not created (it may have been filtered out by duration limit)".to_string(),
            });
        }

        Ok(output_path.to_path_buf())
    }
}

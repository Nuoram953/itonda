use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

pub fn linux_candidate_paths(home: &Path) -> Vec<PathBuf> {
    vec![
        home.join(".local/share/Steam"),
        home.join(".steam/steam"),
        home.join(".steam/root"),
        home.join(".var/app/com.valvesoftware.Steam/data/Steam"),
        home.join(".var/app/com.valvesoftware.Steam/.local/share/Steam"),
        home.join("snap/steam/common/.local/share/Steam"),
        home.join("snap/steam/common/.steam/steam"),
    ]
}

pub fn windows_candidate_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(prog_files_x86) = std::env::var_os("ProgramFiles(x86)") {
        paths.push(PathBuf::from(prog_files_x86).join("Steam"));
    }
    if let Some(prog_files) = std::env::var_os("ProgramFiles") {
        paths.push(PathBuf::from(prog_files).join("Steam"));
    }

    paths.push(PathBuf::from("C:\\Steam"));
    paths.push(PathBuf::from("D:\\Steam"));
    paths.push(PathBuf::from("E:\\Steam"));
    paths.push(PathBuf::from("F:\\Steam"));

    paths
}

pub fn macos_candidate_paths(home: &Path) -> Vec<PathBuf> {
    vec![home.join("Library/Application Support/Steam")]
}

pub fn os_candidate_paths() -> Vec<PathBuf> {
    #[cfg(target_os = "linux")]
    {
        dirs::home_dir()
            .map(|h| linux_candidate_paths(&h))
            .unwrap_or_default()
    }

    #[cfg(target_os = "windows")]
    {
        windows_candidate_paths()
    }

    #[cfg(target_os = "macos")]
    {
        dirs::home_dir()
            .map(|h| macos_candidate_paths(&h))
            .unwrap_or_default()
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        Vec::new()
    }
}
pub fn find_steam_install_dirs() -> Vec<PathBuf> {
    let mut valid_dirs = Vec::new();
    let mut seen_canonical_paths = HashSet::new();

    for path in os_candidate_paths() {
        if !path.exists() || !path.is_dir() {
            continue;
        }

        if !path.join("steamapps").exists() && !path.join("config").exists() {
            continue;
        }

        let canonical = match std::fs::canonicalize(&path) {
            Ok(c) => c,
            Err(_) => path.clone(),
        };

        if seen_canonical_paths.insert(canonical) {
            valid_dirs.push(path);
        }
    }

    valid_dirs
}

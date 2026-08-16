use std::path::PathBuf;

use uuid::Uuid;

#[derive(Clone)]
pub struct AppPaths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
}

impl Default for AppPaths {
    fn default() -> Self {
        Self::new()
    }
}

impl AppPaths {
    pub fn new() -> Self {
        Self {
            config_dir: dirs::config_dir().unwrap().join("itonda-server"),
            data_dir: dirs::data_dir().unwrap().join("itonda-server"),
        }
    }

    pub fn log_dir(&self) -> PathBuf {
        let dir = self.config_dir.join("log");
        if !dir.exists() {
            let _ = std::fs::create_dir_all(&dir);
        }
        dir
    }

    pub fn media_dir(&self, media_id: Uuid) -> PathBuf {
        self.data_dir.join("media").join(media_id.to_string())
    }
}

#[derive(Clone)]
pub struct AgentPaths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
}

impl Default for AgentPaths {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentPaths {
    pub fn new() -> Self {
        Self {
            config_dir: dirs::config_dir().unwrap().join("itonda-agent"),
            data_dir: dirs::data_dir().unwrap().join("itonda-agent"),
        }
    }

    pub fn log_dir(&self) -> PathBuf {
        let dir = self.config_dir.join("log");
        if !dir.exists() {
            let _ = std::fs::create_dir_all(&dir);
        }
        dir
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn log_dir_creates_directory_if_it_does_not_exist() {
        let temp = tempdir().unwrap();
        let paths = AppPaths {
            config_dir: temp.path().join("config"),
            data_dir: temp.path().join("data"),
        };

        let log_dir = paths.log_dir();
        assert!(log_dir.exists());
        assert_eq!(log_dir, temp.path().join("config").join("log"));
    }

    #[test]
    fn agent_paths_log_dir_creates_directory_if_it_does_not_exist() {
        let temp = tempdir().unwrap();
        let paths = AgentPaths {
            config_dir: temp.path().join("config"),
            data_dir: temp.path().join("data"),
        };

        let log_dir = paths.log_dir();
        assert!(log_dir.exists());
        assert_eq!(log_dir, temp.path().join("config").join("log"));
    }
}

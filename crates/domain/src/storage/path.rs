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

    pub fn media_dir(&self, media_id: Uuid) -> PathBuf {
        self.data_dir.join("media").join(media_id.to_string())
    }
}

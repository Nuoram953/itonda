use std::path::PathBuf;

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
            config_dir: dirs::config_dir().unwrap().join("Itonda"),

            data_dir: dirs::data_dir().unwrap().join("Itonda"),
        }
    }
}

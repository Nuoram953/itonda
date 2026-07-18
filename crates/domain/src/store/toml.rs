use std::{fs, path::PathBuf, sync::Arc};

use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::RwLock;
use tracing::{debug, info};

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("toml deserialize error: {0}")]
    Deserialize(#[from] toml::de::Error),

    #[error("toml serialize error: {0}")]
    Serialize(#[from] toml::ser::Error),
}

#[derive(Clone)]
pub struct TomlStore<T> {
    path: PathBuf,
    value: Arc<RwLock<T>>,
}

impl<T> TomlStore<T>
where
    T: Serialize + DeserializeOwned + Clone + Default,
{
    pub fn load(path: PathBuf) -> Result<Self, StoreError> {
        info!(
            path = ?path,
            "Loading configuration"
        );

        let value = if path.exists() {
            debug!("Configuration file found");

            let content = fs::read_to_string(&path)?;

            toml::from_str(&content)?
        } else {
            info!(
                path = ?path,
                "Configuration missing, creating default"
            );

            let value = T::default();

            let content = toml::to_string_pretty(&value)?;

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(&path, content)?;

            value
        };

        Ok(Self {
            path,
            value: Arc::new(RwLock::new(value)),
        })
    }

    pub async fn get(&self) -> T {
        self.value.read().await.clone()
    }

    pub async fn update<F>(&self, f: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut T),
    {
        {
            let mut value = self.value.write().await;

            f(&mut value);
        }

        self.save().await
    }

    pub async fn save(&self) -> Result<(), StoreError> {
        let value = self.value.read().await;

        let content = toml::to_string_pretty(&*value)?;

        fs::write(&self.path, content)?;

        info!(
            path = ?self.path,
            "Configuration saved"
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::{Deserialize, Serialize};
    use tempfile::tempdir;

    #[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
    struct TestConfig {
        #[serde(default)]
        enabled: bool,

        #[serde(default)]
        name: String,

        #[serde(default)]
        port: u16,
    }

    #[tokio::test]
    async fn creates_default_config_when_missing() {
        let dir = tempdir().unwrap();

        let path = dir.path().join("config.toml");

        let store = TomlStore::<TestConfig>::load(path.clone()).unwrap();

        let value = store.get().await;

        assert_eq!(value, TestConfig::default());

        assert!(path.exists());
    }

    #[tokio::test]
    async fn loads_existing_config() {
        let dir = tempdir().unwrap();

        let path = dir.path().join("config.toml");

        fs::write(
            &path,
            r#"
            enabled = true
            name = "Itonda"
            "#,
        )
        .unwrap();

        let store = TomlStore::<TestConfig>::load(path).unwrap();

        let value = store.get().await;

        assert!(value.enabled);

        assert_eq!(value.name, "Itonda");
    }

    #[tokio::test]
    async fn updates_and_saves_config() {
        let dir = tempdir().unwrap();

        let path = dir.path().join("config.toml");

        let store = TomlStore::<TestConfig>::load(path.clone()).unwrap();

        store
            .update(|config| {
                config.enabled = true;
                config.name = "Updated".into();
            })
            .await
            .unwrap();

        let value = store.get().await;

        assert!(value.enabled);

        assert_eq!(value.name, "Updated");

        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("Updated"));
    }

    #[tokio::test]
    async fn missing_fields_use_defaults() {
        let dir = tempdir().unwrap();

        let path = dir.path().join("config.toml");

        fs::write(
            &path,
            r#"
        enabled = true
        "#,
        )
        .unwrap();

        let store = TomlStore::<TestConfig>::load(path).unwrap();

        let value = store.get().await;

        assert!(value.enabled);

        assert_eq!(value.name, "");

        assert_eq!(value.port, 0);
    }

    #[tokio::test]
    async fn ignores_deprecated_fields() {
        let dir = tempdir().unwrap();

        let path = dir.path().join("config.toml");

        fs::write(
            &path,
            r#"
        enabled = true
        name = "Itonda"
        old_name = "Deprecated"
        old_port = 1234
        "#,
        )
        .unwrap();

        let store = TomlStore::<TestConfig>::load(path).unwrap();

        let value = store.get().await;

        assert!(value.enabled);
        assert_eq!(value.name, "Itonda");
        assert_eq!(value.port, 0);
    }
}

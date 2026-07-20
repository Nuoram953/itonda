use std::{fs, path::PathBuf, sync::Arc};

use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::RwLock;

use crate::store::{codec::StoreCodec, error::StoreError};

pub mod codec;
pub mod error;
pub mod toml;

#[derive(Clone)]
pub struct Store<T, C> {
    path: PathBuf,
    codec: C,
    value: Arc<RwLock<T>>,
}

impl<T, C> Store<T, C>
where
    T: Serialize + DeserializeOwned + Clone + Default,
    C: StoreCodec,
{
    pub fn load(path: PathBuf, codec: C) -> Result<Self, StoreError> {
        let value = if path.exists() {
            let content = fs::read(&path)?;

            codec.decode(&content)?
        } else {
            let value = T::default();

            let content = codec.encode(&value)?;

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(&path, content)?;

            value
        };

        Ok(Self {
            path,
            codec,
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

    async fn save(&self) -> Result<(), StoreError> {
        let value = self.value.read().await;

        let content = self.codec.encode(&*value)?;

        fs::write(&self.path, content)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::store::toml::TomlCodec;

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

        type TestStore = Store<TestConfig, TomlCodec>;

        let store = TestStore::load(path.clone(), TomlCodec).unwrap();

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

        type TestStore = Store<TestConfig, TomlCodec>;

        let store = TestStore::load(path.clone(), TomlCodec).unwrap();

        let value = store.get().await;

        assert!(value.enabled);

        assert_eq!(value.name, "Itonda");
    }

    #[tokio::test]
    async fn updates_and_saves_config() {
        let dir = tempdir().unwrap();

        let path = dir.path().join("config.toml");

        type TestStore = Store<TestConfig, TomlCodec>;

        let store = TestStore::load(path.clone(), TomlCodec).unwrap();

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

        type TestStore = Store<TestConfig, TomlCodec>;

        let store = TestStore::load(path.clone(), TomlCodec).unwrap();

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

        type TestStore = Store<TestConfig, TomlCodec>;

        let store = TestStore::load(path.clone(), TomlCodec).unwrap();

        let value = store.get().await;

        assert!(value.enabled);
        assert_eq!(value.name, "Itonda");
        assert_eq!(value.port, 0);
    }
}

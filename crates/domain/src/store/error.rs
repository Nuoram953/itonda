#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("toml deserialize error: {0}")]
    Deserialize(#[from] toml::de::Error),

    #[error("toml serialize error: {0}")]
    Serialize(#[from] toml::ser::Error),
}

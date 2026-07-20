use serde::{Serialize, de::DeserializeOwned};

use crate::store::{codec::StoreCodec, error::StoreError};

#[derive(Clone)]
pub struct TomlCodec;

impl StoreCodec for TomlCodec {
    fn encode<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, StoreError> {
        Ok(toml::to_string_pretty(value)?.into_bytes())
    }

    fn decode<T: DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, StoreError> {
        Ok(toml::from_slice(bytes)?)
    }
}

use serde::{Serialize, de::DeserializeOwned};

use crate::store::error::StoreError;

pub trait StoreCodec: Clone + Send + Sync {
    fn encode<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, StoreError>;

    fn decode<T: DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, StoreError>;
}

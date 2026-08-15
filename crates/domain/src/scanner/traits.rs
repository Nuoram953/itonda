use async_trait::async_trait;

use crate::{
    media::types::MediaType,
    scanner::{errors::ScannerError, models::ScannedMedia},
};

#[async_trait]
pub trait MediaScanner: Send + Sync {
    fn supported_media_type(&self) -> Vec<MediaType>;
    fn name(&self) -> &'static str;
    fn is_available(&self) -> bool {
        true
    }
    async fn scan(&self) -> Result<Vec<ScannedMedia>, ScannerError>;
}

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum StorefrontId {
    Steam,
}

impl From<StorefrontId> for u32 {
    fn from(value: StorefrontId) -> Self {
        match value {
            StorefrontId::Steam => 0,
        }
    }
}

impl StorefrontId {
    pub fn as_steam_grid_db_platform(&self) -> &'static str {
        match self {
            StorefrontId::Steam => "steam",
        }
    }
}

#[derive(Debug, Clone)]
pub struct OwnedGame {
    pub storefront: StorefrontId,
    pub external_id: String,
    pub title: String,
    pub playtime_minutes: Option<u64>,
}

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum StorefrontId {
    Steam,
}

impl StorefrontId {
    pub fn id(&self) -> i64 {
        *self as i64
    }
}

impl From<StorefrontId> for u32 {
    fn from(value: StorefrontId) -> Self {
        match value {
            StorefrontId::Steam => 0,
        }
    }
}

impl StorefrontId {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorefrontId::Steam => "0",
        }
    }

    pub fn as_steam_grid_db_platform(&self) -> &'static str {
        match self {
            StorefrontId::Steam => "steam",
        }
    }

    pub fn as_the_internet_game_database(&self) -> i32 {
        match self {
            StorefrontId::Steam => 1,
        }
    }
}

impl TryFrom<&str> for StorefrontId {
    type Error = crate::storefronts::error::StorefrontError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "steam" | "0" => Ok(StorefrontId::Steam),
            _ => Err(
                crate::storefronts::error::StorefrontError::InvalidStorefrontId(value.to_string()),
            ),
        }
    }
}

impl TryFrom<String> for StorefrontId {
    type Error = crate::storefronts::error::StorefrontError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct OwnedGame {
    pub storefront: StorefrontId,
    pub external_id: String,
    pub title: String,
    pub playtime_minutes: Option<u64>,
}

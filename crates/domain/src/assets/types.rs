use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::assets::error::AssetError;

#[repr(i64)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum AssetType {
    Poster = 1,
    Backdrop = 2,
    Logo = 3,
    Banner = 4,
    Thumbnail = 5,
    Icon = 6,
    Trailer = 7,
    Screenshot = 8,
}

impl AssetType {
    pub fn id(self) -> i64 {
        self as i64
    }

    pub fn folder(&self) -> &'static str {
        match self {
            AssetType::Poster => "poster",
            AssetType::Backdrop => "backdrop",
            AssetType::Logo => "logo",
            AssetType::Banner => "banner",
            AssetType::Thumbnail => "thumbnail",
            AssetType::Icon => "icon",
            AssetType::Trailer => "trailer",
            AssetType::Screenshot => "screenshot",
        }
    }
}

impl TryFrom<i64> for AssetType {
    type Error = AssetError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(AssetType::Poster),
            2 => Ok(AssetType::Backdrop),
            3 => Ok(AssetType::Logo),
            4 => Ok(AssetType::Banner),
            5 => Ok(AssetType::Thumbnail),
            6 => Ok(AssetType::Icon),
            7 => Ok(AssetType::Trailer),
            8 => Ok(AssetType::Screenshot),
            _ => Err(AssetError::InvalidAssetType),
        }
    }
}

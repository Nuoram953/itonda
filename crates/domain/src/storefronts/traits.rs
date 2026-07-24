use async_trait::async_trait;

use crate::{
    media::models::DiscoveredMedia,
    storefronts::{error::StorefrontError, models::StorefrontId},
};

#[async_trait]
pub trait Storefront {
    fn id(&self) -> StorefrontId;
}

#[async_trait]
pub trait GameLibraryProvider: Storefront + Send + Sync {
    async fn owned_games(&self) -> Result<Vec<DiscoveredMedia>, StorefrontError>;
}

// #[async_trait]
// pub trait AchievementProvider {
//     async fn achievements(&self, game_id: &str) -> Result<Vec<Achievement>, StorefrontError>;
// }

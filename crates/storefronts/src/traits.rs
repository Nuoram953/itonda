use async_trait::async_trait;
use itonda_domain::storefront::models::StorefrontId;

use crate::{error::StorefrontError, models::OwnedGame};

#[async_trait]
pub trait Storefront {
    fn id(&self) -> StorefrontId;
}

#[async_trait]
pub trait GameLibraryProvider: Storefront + Send + Sync {
    async fn owned_games(&self) -> Result<Vec<OwnedGame>, StorefrontError>;
}

// #[async_trait]
// pub trait AchievementProvider {
//     async fn achievements(&self, game_id: &str) -> Result<Vec<Achievement>, StorefrontError>;
// }

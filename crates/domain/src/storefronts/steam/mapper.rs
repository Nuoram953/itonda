use crate::{
    media::models::{DiscoveredMedia, DiscoveredMediaMetadata, MediaType},
    storefronts::{models::StorefrontId, steam::models::SteamApp},
};

pub fn map_owned_game(app: SteamApp) -> DiscoveredMedia {
    DiscoveredMedia {
        storefront: StorefrontId::Steam,
        external_id: app.appid.to_string(),
        title: app.name,
        metadata: DiscoveredMediaMetadata {
            total_playtime: Some(app.playtime_forever.unwrap_or(0)),
        },
        media_type: MediaType::Game,
    }
}

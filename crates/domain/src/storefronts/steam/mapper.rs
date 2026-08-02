use crate::{
    media::{
        discovered::{DiscoveredLaunch, DiscoveredMedia, DiscoveredMediaMetadata, GameMetadata},
        types::{MediaLaunchType, MediaType},
    },
    storefronts::{models::StorefrontId, steam::models::SteamApp},
};

pub fn map_owned_game(app: SteamApp) -> DiscoveredMedia {
    DiscoveredMedia {
        storefront: StorefrontId::Steam,
        external_id: app.appid.to_string(),
        title: app.name,
        metadata: DiscoveredMediaMetadata::Game(GameMetadata {
            total_playtime: Some(app.playtime_forever.unwrap_or(0)),
            last_played: Some(app.rtime_last_played.unwrap_or(0)),
        }),
        media_type: MediaType::Game,
        launch: Some(DiscoveredLaunch {
            name: "Steam".into(),
            launch_type: MediaLaunchType::Storefront,
            program: "steam".into(),
            arguments: vec![format!("steam://run/{}", app.appid)],
            working_directory: None,
        }),
    }
}

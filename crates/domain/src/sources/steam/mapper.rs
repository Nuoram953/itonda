use crate::{
    media::{
        discovered::{DiscoveredLaunch, DiscoveredMedia, DiscoveredMediaMetadata, GameMetadata},
        types::{MediaLaunchType, MediaType},
    },
    sources::steam::models::SteamApp,
    storefronts::models::StorefrontId,
};

pub fn map_owned_game(game: SteamApp) -> DiscoveredMedia {
    DiscoveredMedia {
        storefront: StorefrontId::Steam,
        external_id: game.appid.to_string(),
        media_type: MediaType::Game,
        title: game.name,
        metadata: DiscoveredMediaMetadata::Game(GameMetadata {
            total_playtime: game.playtime_forever,
            last_played: game.rtime_last_played,
        }),
        launch: Some(DiscoveredLaunch {
            name: "Steam".into(),
            launch_type: MediaLaunchType::Storefront,
            program: "steam".into(),
            arguments: vec![format!("steam://run/{}", game.appid)],
            working_directory: None,
        }),
    }
}

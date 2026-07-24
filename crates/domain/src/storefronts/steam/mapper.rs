use crate::storefronts::{
    models::{OwnedGame, StorefrontId},
    steam::models::SteamApp,
};

pub fn map_owned_game(app: SteamApp) -> OwnedGame {
    OwnedGame {
        storefront: StorefrontId::Steam,
        external_id: app.appid.to_string(),
        title: app.name,
        playtime_minutes: Some(app.playtime_forever.unwrap_or(0)),
    }
}

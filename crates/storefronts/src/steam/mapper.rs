use itonda_domain::storefront::models::StorefrontId;

use crate::{models::OwnedGame, steam::models::SteamApp};

pub fn map_owned_game(app: SteamApp) -> OwnedGame {
    OwnedGame {
        storefront: StorefrontId::Steam,
        external_id: app.appid.to_string(),
        title: app.name,
        playtime_minutes: Some(app.playtime_forever.unwrap_or(0)),
    }
}

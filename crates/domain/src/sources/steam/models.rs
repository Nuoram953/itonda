use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetOwnedGamesResponse {
    pub response: OwnedGamesPayload,
}

#[derive(Debug, Deserialize)]
pub struct OwnedGamesPayload {
    pub games: Option<Vec<SteamApp>>,
}

#[derive(Debug, Deserialize)]
pub struct SteamApp {
    pub appid: u64,
    pub name: String,
    pub playtime_forever: Option<u64>,
    pub rtime_last_played: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct GetPlayerSummariesResponse {
    pub response: PlayerSummariesPayload,
}

#[derive(Debug, Deserialize)]
pub struct PlayerSummariesPayload {
    pub players: Option<Vec<SteamPlayerSummary>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct SteamPlayerSummary {
    pub steamid: String,
    pub personaname: Option<String>,
    pub profileurl: Option<String>,
    pub avatar: Option<String>,
    pub avatarmedium: Option<String>,
    pub avatarfull: Option<String>,
}

use crate::{
    assets::types::AssetType,
    media::types::{MediaLaunchType, MediaType},
    storefronts::models::StorefrontId,
};

#[derive(Clone, Debug)]
pub struct DiscoveredMedia {
    pub storefront: StorefrontId,
    pub external_id: String,
    pub media_type: MediaType,
    pub title: String,
    pub metadata: DiscoveredMediaMetadata,
    pub launch: Option<DiscoveredLaunch>,
}

#[derive(Clone, Debug)]
pub struct DiscoveredLaunch {
    pub name: String,
    pub launch_type: MediaLaunchType,
    pub program: String,
    pub arguments: Vec<String>,
    pub working_directory: Option<String>,
}

#[derive(Clone, Debug)]
pub enum DiscoveredMediaMetadata {
    Game(GameMetadata),
}

#[derive(Clone, Debug)]
pub struct GameMetadata {
    pub total_playtime: Option<u64>,
    pub last_played: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct DiscoveredAsset {
    pub asset_type: AssetType,
    pub url: String,
}

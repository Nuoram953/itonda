use crate::{
    assets::types::AssetType,
    media::{
        models::MediaExternalId,
        types::{MediaLaunchType, MediaType},
    },
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscoveredAsset {
    pub asset_type: AssetType,
    pub url: String,
    pub provider_external_id: Option<MediaExternalId>,
    pub pillar_id: Option<String>,
}

impl DiscoveredAsset {
    pub fn new(asset_type: AssetType, url: impl Into<String>) -> Self {
        Self {
            asset_type,
            url: url.into(),
            provider_external_id: None,
            pillar_id: None,
        }
    }

    pub fn with_provider_external_id(mut self, external_id: MediaExternalId) -> Self {
        self.provider_external_id = Some(external_id);
        self
    }

    pub fn with_pillar_id(mut self, pillar_id: impl Into<String>) -> Self {
        self.pillar_id = Some(pillar_id.into());
        self
    }
}

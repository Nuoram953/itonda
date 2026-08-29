use std::collections::{HashMap, HashSet};

use crate::media::models::MediaExternalId;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetStoreId {
    SteamGridDb,
    TheMovieDatabase,
}

impl From<AssetStoreId> for u32 {
    fn from(value: AssetStoreId) -> Self {
        match value {
            AssetStoreId::SteamGridDb => 0,
            AssetStoreId::TheMovieDatabase => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PosterSearchOptions {
    SteamGridDb(crate::sources::steam_grid_db::models::GridSearchOptions),
    Default,
}

#[derive(Debug, Clone)]
pub struct DiscoverOptions<'a> {
    pub existing_counts: &'a HashMap<i64, usize>,
    pub searched_types: &'a HashSet<i64>,
    pub limit: Option<usize>,
    pub force: bool,
    pub external_ids: &'a [MediaExternalId],
}

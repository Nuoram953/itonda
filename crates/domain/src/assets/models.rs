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
    SteamGridDb(crate::assets::steam_grid_db::models::GridSearchOptions),
    Default,
}

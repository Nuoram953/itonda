use itonda_domain::storefront::models::StorefrontId;

#[derive(Debug, Clone)]
pub struct OwnedGame {
    pub storefront: StorefrontId,
    pub external_id: String,
    pub title: String,
    pub playtime_minutes: Option<u64>,
}

use crate::{
    media::discovered::DiscoveredMedia, sync::context::SyncContext,
    tests::fixtures::media::DiscoveredMediaBuilder,
};

pub fn sync_context() -> SyncContext {
    sync_context_with_media(DiscoveredMediaBuilder::new().build())
}

pub fn sync_context_with_media(media: DiscoveredMedia) -> SyncContext {
    SyncContext::new(media)
}

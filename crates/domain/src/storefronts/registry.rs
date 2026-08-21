use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::storefronts::{models::StorefrontId, traits::GameLibraryProvider};

#[derive(Clone)]
pub struct StorefrontRegistry {
    providers: Arc<RwLock<HashMap<StorefrontId, Arc<dyn GameLibraryProvider>>>>,
}

impl Default for StorefrontRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl StorefrontRegistry {
    pub fn new() -> Self {
        Self {
            providers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register(&self, provider: Arc<dyn GameLibraryProvider>) {
        self.providers
            .write()
            .unwrap()
            .insert(provider.id(), provider);
    }

    pub fn remove(&self, id: StorefrontId) -> Option<Arc<dyn GameLibraryProvider>> {
        self.providers.write().unwrap().remove(&id)
    }

    pub fn get(&self, id: StorefrontId) -> Option<Arc<dyn GameLibraryProvider>> {
        self.providers.read().unwrap().get(&id).cloned()
    }

    pub fn get_all(&self) -> HashMap<StorefrontId, Arc<dyn GameLibraryProvider>> {
        self.providers.read().unwrap().clone()
    }

    pub fn available(&self) -> Vec<StorefrontId> {
        self.providers.read().unwrap().keys().copied().collect()
    }
}

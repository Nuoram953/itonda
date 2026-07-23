use std::{collections::HashMap, sync::Arc};

use itonda_domain::storefront::models::StorefrontId;

use crate::traits::GameLibraryProvider;

#[derive(Clone)]
pub struct StorefrontRegistry {
    providers: HashMap<StorefrontId, Arc<dyn GameLibraryProvider>>,
}

impl StorefrontRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn register(&mut self, provider: Arc<dyn GameLibraryProvider>) {
        self.providers.insert(provider.id(), provider);
    }

    pub fn get(&self, id: StorefrontId) -> Option<Arc<dyn GameLibraryProvider>> {
        self.providers.get(&id).cloned()
    }

    pub fn available(&self) -> Vec<StorefrontId> {
        self.providers.keys().copied().collect()
    }
}

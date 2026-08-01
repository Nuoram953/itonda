#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AssetPolicy {
    All,
    #[default]
    FirstOnly,
    Limit(usize),
}

impl AssetPolicy {
    pub fn max_items(&self) -> Option<usize> {
        match self {
            Self::All => None,
            Self::FirstOnly => Some(1),
            Self::Limit(limit) => Some(*limit),
        }
    }

    pub fn apply<T>(&self, mut items: Vec<T>) -> Vec<T> {
        if let Some(limit) = self.max_items() {
            items.truncate(limit);
        }
        items
    }
}

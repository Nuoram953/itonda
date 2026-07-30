#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpsertAction {
    Created,
    Updated,
    Unchanged,
}

pub struct UpsertResult<T> {
    pub value: T,
    pub action: UpsertAction,
}

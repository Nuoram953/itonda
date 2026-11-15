use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow, PartialEq, Eq)]
pub struct EmulatorRow {
    pub id: String,
    pub name: String,
    pub program: String,
    pub arguments: String,
    pub working_directory: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct EmulatorInsert {
    pub name: String,
    pub program: String,
    pub arguments: String,
    pub working_directory: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EmulatorUpdate {
    pub name: String,
    pub program: String,
    pub arguments: String,
    pub working_directory: Option<String>,
}

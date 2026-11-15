use itonda_database::emulators::models::EmulatorRow;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct Emulator {
    pub id: String,
    pub name: String,
    pub program: String,
    pub arguments: Vec<String>,
    pub working_directory: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<EmulatorRow> for Emulator {
    fn from(row: EmulatorRow) -> Self {
        let arguments = serde_json::from_str(&row.arguments).unwrap_or_default();
        Self {
            id: row.id,
            name: row.name,
            program: row.program,
            arguments,
            working_directory: row.working_directory,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

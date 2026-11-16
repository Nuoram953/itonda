use itonda_database::emulators::models::EmulatorRow;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum EmulatorOptionType {
    #[default]
    Flag,
    Value,
    Select,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct EmulatorOption {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "type", default)]
    pub option_type: EmulatorOptionType,
    pub flag: String,
    #[serde(default)]
    pub default_enabled: bool,
    #[serde(default)]
    pub default_value: Option<String>,
    #[serde(default)]
    pub choices: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct EmulatorDefinition {
    pub id: String,
    pub name: String,
    pub default_program: String,
    #[serde(default)]
    pub default_arguments: Vec<String>,
    #[serde(default)]
    pub options: Vec<EmulatorOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct Emulator {
    pub id: String,
    pub name: String,
    pub program: String,
    pub arguments: Vec<String>,
    pub options: Vec<EmulatorOption>,
    pub working_directory: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<EmulatorRow> for Emulator {
    fn from(row: EmulatorRow) -> Self {
        let arguments = serde_json::from_str(&row.arguments).unwrap_or_default();
        let options = serde_json::from_str(&row.options).unwrap_or_default();
        Self {
            id: row.id,
            name: row.name,
            program: row.program,
            arguments,
            options,
            working_directory: row.working_directory,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

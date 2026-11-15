use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEmulatorPayload {
    pub name: String,
    pub program: String,
    #[serde(default)]
    pub arguments: Vec<String>,
    pub working_directory: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateEmulatorPayload {
    pub name: Option<String>,
    pub program: Option<String>,
    pub arguments: Option<Vec<String>>,
    pub working_directory: Option<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetEmulatorsResponse {
    pub items: Vec<itonda_domain::emulators::models::Emulator>,
}

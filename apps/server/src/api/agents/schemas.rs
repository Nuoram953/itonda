use itonda_domain::agents::models::Agent;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetAgentsResponse {
    pub agents: Vec<Agent>,
}

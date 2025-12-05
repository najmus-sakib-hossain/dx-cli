use serde::{Deserialize, Serialize};

pub type ResourceId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDefinition {
    pub id: ResourceId,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceContent {
    pub id: ResourceId,
    pub bytes: Vec<u8>,
    pub content_type: String,
}

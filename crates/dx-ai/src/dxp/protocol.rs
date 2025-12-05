use serde::{Deserialize, Serialize};

use crate::dxp::resources::{ResourceContent, ResourceDefinition, ResourceId};
use crate::dxp::tools::{ToolCall, ToolDefinition};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DxpMessage {
    ListTools,
    ToolsResponse(Vec<ToolDefinition>),
    CallTool(ToolCall),
    ToolResult(ToolResult),
    ToolError(String),
    ListResources,
    ResourcesResponse(Vec<ResourceDefinition>),
    ReadResource(ResourceId),
    ResourceContent(ResourceContent),
    Subscribe(String),
    Notification(String),
    Unsubscribe(String),
    UpdateContext(serde_json::Value),
    ContextUpdated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub call_id: String,
    pub output: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
}

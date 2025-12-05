use async_trait::async_trait;
use futures::stream::BoxStream;

use crate::friday::models::{AgentDefinition, AgentResult, Task};

#[derive(Debug)]
pub enum ExecutionEvent {
    Started { task_id: String },
    Step { step: u32, action: String },
    ToolCall { tool: String, args: serde_json::Value },
    ToolResult { result: serde_json::Value },
    Thinking { content: String },
    Response { content: String },
    Completed { result: AgentResult },
    Error { error: String },
}

#[async_trait]
pub trait AgentOrchestrator: Send + Sync {
    async fn register_agent(&self, agent: AgentDefinition) -> anyhow::Result<()>;
    async fn execute(&self, agent_id: &str, task: Task) -> anyhow::Result<AgentResult>;
    async fn execute_multi(&self, task: Task, agents: &[String]) -> anyhow::Result<AgentResult>;
    async fn execute_stream(
        &self,
        agent_id: &str,
        task: Task,
    ) -> anyhow::Result<BoxStream<'static, ExecutionEvent>>;
    async fn cancel(&self, task_id: &str) -> anyhow::Result<()>;
}

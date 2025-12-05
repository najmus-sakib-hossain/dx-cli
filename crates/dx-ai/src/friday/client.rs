use async_trait::async_trait;

use crate::friday::models::{AgentDefinition, AgentResult, ChatChunk, ChatRequest, ChatResponse, Message, Model, Task};
use crate::dxp::transport::DxpTransport;
use anyhow::Result;
use futures::stream::BoxStream;

#[async_trait]
pub trait FridayClient: Send + Sync {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;
    async fn chat_stream(&self, request: ChatRequest) -> Result<BoxStream<'static, Result<ChatChunk>>>;
    async fn execute_agent(&self, agent: AgentDefinition, task: Task) -> Result<AgentResult>;
    async fn list_models(&self) -> Result<Vec<Model>>;
}

/// Basic placeholder client that only ensures the trait is object-safe.
pub struct NoopFridayClient<T: DxpTransport> {
    pub transport: T,
}

#[async_trait]
impl<T> FridayClient for NoopFridayClient<T>
where
    T: DxpTransport + Send + Sync,
{
    async fn chat(&self, _request: ChatRequest) -> Result<ChatResponse> {
        Ok(ChatResponse {
            message: Message::assistant_text("stub response"),
            model: None,
        })
    }

    async fn chat_stream(&self, _request: ChatRequest) -> Result<BoxStream<'static, Result<ChatChunk>>> {
        use futures::StreamExt;
        let stream = futures::stream::iter(vec![Ok(ChatChunk::text("stub".into()))]).boxed();
        Ok(stream)
    }

    async fn execute_agent(&self, _agent: AgentDefinition, _task: Task) -> Result<AgentResult> {
        Ok(AgentResult {
            content: "stub agent result".to_string(),
            metadata: None,
        })
    }

    async fn list_models(&self) -> Result<Vec<Model>> {
        Ok(vec![])
    }
}

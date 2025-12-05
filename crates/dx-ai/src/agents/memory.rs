use async_trait::async_trait;

use crate::friday::models::Message;
use crate::context::Context;

#[async_trait]
pub trait AiMemory: Send + Sync {
    async fn store_conversation(&self, id: &str, messages: &[Message]) -> anyhow::Result<()>;
    async fn get_conversation(&self, id: &str) -> anyhow::Result<Option<Vec<Message>>>;
    async fn store_context(&self, context: &Context) -> anyhow::Result<()>;
    async fn get_relevant_context(&self, query: &str, limit: usize) -> anyhow::Result<Vec<ContextChunk>>;
    async fn clear(&self) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub struct ContextChunk {
    pub title: String,
    pub content: String,
}

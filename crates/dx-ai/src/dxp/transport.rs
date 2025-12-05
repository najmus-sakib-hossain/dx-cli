use async_trait::async_trait;

use crate::dxp::protocol::DxpMessage;

#[async_trait]
pub trait DxpTransport: Send + Sync {
    async fn send(&self, message: DxpMessage) -> anyhow::Result<()>;
    async fn receive(&self) -> anyhow::Result<DxpMessage>;
    async fn close(&self) -> anyhow::Result<()>;
}

/// A no-op transport useful for tests and stubs.
pub struct NoopTransport;

#[async_trait]
impl DxpTransport for NoopTransport {
    async fn send(&self, _message: DxpMessage) -> anyhow::Result<()> {
        Ok(())
    }

    async fn receive(&self) -> anyhow::Result<DxpMessage> {
        Err(anyhow::anyhow!("No messages available in NoopTransport"))
    }

    async fn close(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

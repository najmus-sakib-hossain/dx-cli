use crate::friday::models::{AgentDefinition, AgentResult, Task};

#[derive(Debug, Default)]
pub struct AgentExecutor;

impl AgentExecutor {
    pub async fn execute(&self, _agent: &AgentDefinition, _task: &Task) -> anyhow::Result<AgentResult> {
        Ok(AgentResult {
            content: "stub executor response".to_string(),
            metadata: None,
        })
    }
}

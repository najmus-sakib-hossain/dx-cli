use async_trait::async_trait;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct GeneratorContext {
    pub variables: serde_json::Value,
    pub dry_run: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GeneratorOutput {
    pub files: Vec<GeneratedFile>,
    pub imports: Vec<String>,
    pub dependencies: Vec<String>,
    pub post_actions: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GeneratedFile {
    pub path: std::path::PathBuf,
    pub content: String,
    pub overwrite: bool,
}

#[async_trait]
pub trait Generator: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn variables(&self) -> Vec<String>;

    async fn generate(&self, context: &GeneratorContext) -> anyhow::Result<GeneratorOutput>;
    async fn preview(&self, context: &GeneratorContext) -> anyhow::Result<GeneratorOutput>;
}

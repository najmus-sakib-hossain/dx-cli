use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DxConfig {
    #[serde(default)]
    pub dx: DxSection,
    #[serde(default)]
    pub cli: CliSection,
    #[serde(default)]
    pub ai: AiSection,
    #[serde(default)]
    pub shell: ShellSection,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DxSection {
    #[serde(default = "default_version")]
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CliSection {
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub verbose: bool,
    #[serde(default = "default_parallel_jobs")]
    pub parallel_jobs: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AiSection {
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    #[serde(default)]
    pub memory_enabled: bool,
    #[serde(default)]
    pub memory_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShellSection {
    #[serde(default)]
    pub autocomplete: bool,
    #[serde(default = "default_fuzzy_threshold")]
    pub fuzzy_threshold: f64,
    #[serde(default = "default_history_size")]
    pub history_size: usize,
    #[serde(default)]
    pub ai_suggestions: bool,
}

fn default_version() -> String {
    "0.1.0".to_string()
}

fn default_parallel_jobs() -> u8 {
    4
}

fn default_temperature() -> f32 {
    0.7
}

fn default_max_tokens() -> u32 {
    4096
}

fn default_fuzzy_threshold() -> f64 {
    0.6
}

fn default_history_size() -> usize {
    10_000
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteConfig {
    pub enabled: bool,
    pub fuzzy_threshold: f64,
    pub max_suggestions: usize,
    pub include_history: bool,
    pub include_files: bool,
    pub include_commands: bool,
    pub ai_suggestions: bool,
}

impl Default for AutocompleteConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            fuzzy_threshold: 0.6,
            max_suggestions: 10,
            include_history: true,
            include_files: true,
            include_commands: true,
            ai_suggestions: true,
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Context {
    pub project_root: Option<String>,
    pub files: Vec<String>,
    pub notes: Vec<String>,
}

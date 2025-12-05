use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum DxError {
    #[error("Configuration error: {message}")]
    Config { message: String, path: Option<PathBuf> },

    #[error("AI service error: {0}")]
    Ai(String),

    #[error("Template error: {0}")]
    Template(String),

    #[error("File system error: {path}")]
    FileSystem {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Network error: {0}")]
    Network(String),

    #[error("DXP protocol error: {0}")]
    Dxp(String),

    #[error("Command failed: {command}")]
    Command { command: String, exit_code: i32 },

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {resource_type} '{name}'")]
    NotFound { resource_type: String, name: String },
}

pub type DxResult<T> = Result<T, DxError>;

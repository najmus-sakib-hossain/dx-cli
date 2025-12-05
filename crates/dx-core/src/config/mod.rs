//! Configuration management for Dx.

pub mod loader;
pub mod schema;
pub mod validation;

pub use loader::{load, load_from_path};
pub use schema::DxConfig;

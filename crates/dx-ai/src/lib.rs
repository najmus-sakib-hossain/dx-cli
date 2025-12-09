#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

pub mod friday;
pub mod agents;
pub mod dxp;
pub mod context;
pub mod gemini;

pub use gemini::{GeminiClient, Message, Role};

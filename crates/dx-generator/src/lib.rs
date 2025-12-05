#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

pub mod engine;
pub mod templates;
pub mod generators;

pub use engine::{GeneratorContext, GeneratorOutput};

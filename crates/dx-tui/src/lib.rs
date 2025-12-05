#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

pub mod app;
pub mod components;
pub mod layouts;
pub mod themes;
pub mod events;

pub use app::TuiApp;

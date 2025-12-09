#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

pub mod app;
pub mod components;
pub mod layouts;
pub mod themes;
pub mod events;
pub mod chat;

pub use app::TuiApp;
pub use chat::ChatApp;

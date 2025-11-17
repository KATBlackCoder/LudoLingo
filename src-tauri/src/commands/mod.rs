// Commands module exports
// This module contains Tauri commands exposed to the frontend

pub mod injection;
pub mod projects;
pub mod scanning;
pub mod translation;
pub mod updater;
pub mod validation;

// Re-export all commands
pub use injection::*;
pub use projects::*;
pub use scanning::*;
pub use translation::*;
#[cfg(desktop)]
pub use updater::*;
pub use validation::*;

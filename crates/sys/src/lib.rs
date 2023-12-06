mod windows;
mod macos;
mod linux;

pub mod event;
pub mod style;
mod window;
pub use window::Window;
pub mod prelude;
pub mod modal;
pub mod error;
mod error;
pub mod modules;
pub mod powerline;
pub mod terminal;
pub mod theme;
#[cfg(target_os = "windows")]
pub mod windows;

pub(crate) mod utils;

pub type R<T> = Result<T, error::Error>;
pub use crate::powerline::{Powerline, Segment};
pub use error::Error;

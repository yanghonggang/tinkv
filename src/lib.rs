//! A simple key-value storage.
pub mod config;
mod error;
mod segment;
mod store;
pub mod util;

pub use error::{Result, TinkvError};
pub use store::{OpenOptions, Store};

mod error;
mod kv;
pub mod log;
pub use crate::error::Error;
pub use crate::kv::{KvStore, Result};

mod error;
mod kv;
pub mod sstable;
pub use crate::error::Error;
pub use crate::kv::{KvStore, Result};

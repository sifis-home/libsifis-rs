mod discovery;
mod thing;

pub mod error {
    pub use anyhow::{Error, Result};
}

pub use discovery::*;
pub use thing::*;

mod discovery;
mod ontology;
mod thing;

pub mod error {
    pub use anyhow::{Error, Result};
}

pub use discovery::*;
pub use ontology::*;
pub use thing::*;

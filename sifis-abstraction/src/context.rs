use std::time::Duration;

use anyhow::{anyhow, Result};
use sifis::{Discovery, Thing};

use crate::ConnectedObject;

/// All information needed to interact with the Connected Things
#[derive(Default)]
pub struct Context {
    #[allow(dead_code)]
    /// All methods and protocols used to discover the Connected Things
    discovery: Discovery,
}

impl Context {
    /// Retrieves all the objects of a certain type.
    pub fn _find_all<T: ConnectedObject + From<Thing>>(&self) -> Result<Vec<T>> {
        let all = self
            .discovery
            .discover_timeout(Duration::from_secs(2))?
            .into_iter()
            .filter(|t| t.attype.contains(&T::AT_TYPE.to_owned()))
            .map(|t| t.into())
            .collect();
        Ok(all)
    }
    /// Retrieves an object of a certain type using a specific identifier.
    pub fn _find<T: ConnectedObject + From<Thing>>(&self, id: &str) -> Result<T> {
        self.discovery
            .discover_timeout(Duration::from_secs(2))?
            .into_iter()
            .filter(|co| co.attype.contains(&T::AT_TYPE.to_owned()))
            .find(|co| co.id == id)
            .map(|t| t.into())
            .ok_or_else(|| anyhow!("Not found!"))
    }
}

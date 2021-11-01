use std::collections::HashMap;

use url::Url;

pub mod error {
    pub use anyhow::{Error, Result};
}

/// SIFIS Hazard Label
///
/// Describes a possible hazard.
// TODO: Support range
pub struct Label {}

pub trait WithLabel {
    fn has_label(&self, label: &Label) -> bool;
}

pub enum Op {}

pub struct Form {
    op: Vec<Op>,
    href: Url,
}

pub struct Affordance {
    label: Vec<Label>,
    title: String,
    description: String,
    forms: Vec<Form>,
}

pub struct Property {
    inner: Affordance,
}

pub struct Action {
    inner: Affordance,
}

pub struct Event {
    inner: Affordance,
}

/// Connected thing
pub struct Thing {
    properties: HashMap<String, Property>,
    actions: HashMap<String, Action>,
    events: HashMap<String, Event>,
}

impl Thing {
    fn properties<'s>(&self) -> impl Iterator<Item = (&String, &Property)> {
        self.properties.iter()
    }
    fn actions<'s>(&self) -> impl Iterator<Item = (&String, &Action)> {
        self.actions.iter()
    }
    fn events<'s>(&self) -> impl Iterator<Item = (&String, &Event)> {
        self.events.iter()
    }
}

/// Store here pre-defined constraints
/// to the access to SIFIS-Home
#[derive(Debug)]
struct Restrictions {}

/// Point of truth to access Things as consumer
#[derive(Debug)]
pub struct Context {
    restrictions: Restrictions,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            restrictions: Restrictions {},
        }
    }
}

pub struct ThingsIter {}

impl Iterator for ThingsIter {
    type Item = Thing;

    fn next(&mut self) -> Option<Thing> {
        todo!()
    }
}

impl Context {
    fn things() -> ThingsIter {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use std::collections::HashMap;

use url::Url;

pub mod error {
    pub use anyhow::{Error, Result};
}

/// SIFIS Hazard Label
///
/// Describes a possible hazard.
///
/// A risk score can *only* assume values in the range [0, 10]. Values outside
/// of the defined range are invalid.
#[derive(Clone)]
pub struct Label {
    name: String,
    description: String,
    risk_score: Option<usize>,
}

impl Label {
    pub fn has_valid_risk_score(&self) -> bool {
        self.risk_score.map_or(false, |v| (0..11).contains(&v))
    }
}

pub trait WithLabel {
    fn has_label(&self, label: &Label) -> bool;
}

#[derive(Clone)]
pub enum Op {
    ReadProperty,
    WriteProperty,
    ObserveProperty,
    UnobserveProperty,
    InvokeAction,
    SubscribeEvent,
    UnsubscribeEvent,
    ReadAllProperties,
    WriteAllProperties,
    ReadMultipleProperties,
    WriteMultipleProperties,
}

#[derive(Clone)]
pub enum SubProtocol {
    Longpoll,
    Websub,
    Sse,
}

#[derive(Clone)]
pub struct ExpectedResponse {
    contentType: String,
}

#[derive(Clone)]
pub struct Form {
    op: Vec<Op>,
    href: Url,
    contentType: String,
    contentCoding: Option<String>,
    subprotocol: Option<SubProtocol>,
    security: Option<Vec<String>>,
    scopes: Option<Vec<String>>,
    response: Option<ExpectedResponse>,
}

#[derive(Clone)]
pub struct Affordance {
    label: Vec<Label>,
    title: String,
    description: String,
    forms: Vec<Form>,
}

#[derive(Clone)]
pub struct Property {
    inner: Affordance,
}

#[derive(Clone)]
pub struct Action {
    inner: Affordance,
}

#[derive(Clone)]
pub struct Event {
    inner: Affordance,
}

/// Connected thing
#[derive(Clone)]
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

/// Point of truth to access Things as consumer
pub struct Context {
    things: Vec<Thing>,
}

impl Context {
    /// Creates a new Context composed by a series of Thing.
    pub fn new(things: &[Thing]) -> Self {
        Self {
            things: things.to_owned(),
        }
    }
    /// Returns an Iterator over the Thing.
    pub fn things(&self) -> impl Iterator<Item = &Thing> {
        self.things.iter()
    }
}

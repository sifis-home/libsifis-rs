use std::collections::HashMap;

use serde::Deserialize;

use crate::label::Label;

/*
#[derive(Clone, Deserialize, Debug)]
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

#[derive(Clone, Deserialize, Debug)]
pub enum SubProtocol {
    Longpoll,
    Websub,
    Sse,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ExpectedResponse {
    contentType: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Form {
    op: Vec<Op>,
    href: String,
    contentType: String,
    contentCoding: Option<String>,
    subprotocol: Option<SubProtocol>,
    security: Option<Vec<String>>,
    scopes: Option<Vec<String>>,
    response: Option<ExpectedResponse>,
}
*/

#[derive(Clone, Deserialize, Debug)]
pub struct Affordance {
    #[serde(default, rename = "sifis:hazard")]
    hazard: serde_json::Value,
    // title: String,
    // description: String,
    //    forms: Vec<Form>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Property {
    #[serde(flatten)]
    inner: Affordance,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Action {
    #[serde(flatten)]
    inner: Affordance,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Event {
    #[serde(flatten)]
    inner: Affordance,
}

/// Connected thing
#[derive(Clone, Debug, Deserialize)]
pub struct Thing {
    pub title: String,
    #[serde(rename = "@type")]
    pub attype: Vec<String>,
    properties: HashMap<String, Property>,
    actions: HashMap<String, Action>,
    events: HashMap<String, Event>,
}

impl Thing {
    pub fn properties<'s>(&self) -> impl Iterator<Item = (&String, &Property)> {
        self.properties.iter()
    }
    pub fn actions<'s>(&self) -> impl Iterator<Item = (&String, &Action)> {
        self.actions.iter()
    }
    pub fn events<'s>(&self) -> impl Iterator<Item = (&String, &Event)> {
        self.events.iter()
    }
}

use std::collections::HashMap;

use serde::Deserialize;

use crate::label::Label;

#[derive(Clone, Deserialize, Debug)]
pub struct DataSchema {
    pub maximum: Option<usize>,
    pub minimum: Option<usize>,
    pub title: Option<String>,
    pub r#type: Option<String>,
    pub unit: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Affordance {
    /*#[serde(default, rename = "sifis:hazard")]
    hazard: serde_json::Value,*/
    #[serde(rename = "@type")]
    pub attype: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "links")]
    pub forms: Vec<Form>,
    pub titles: Option<HashMap<String, String>>,
    pub descriptions: Option<HashMap<String, String>>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Property {
    #[serde(flatten)]
    pub affordance: Affordance,
    pub observable: Option<bool>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Action {
    #[serde(flatten)]
    pub affordance: Affordance,
    pub input: Option<DataSchema>,
    pub output: Option<DataSchema>,
    #[serde(default)]
    pub safe: bool,
    #[serde(default)]
    pub idempotent: bool,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Event {
    #[serde(flatten)]
    pub affordance: Affordance,
    pub subscription: Option<DataSchema>,
    pub data: Option<DataSchema>,
    pub cancellation: Option<DataSchema>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SecurityScheme {
    #[serde(flatten)]
    pub scheme: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Form {
    pub href: String,
    pub rel: String,
}

/// Connected thing
#[derive(Clone, Debug, Deserialize)]
pub struct Thing {
    #[serde(rename = "@context")]
    pub context: String,
    pub title: String,
    #[serde(rename = "@type")]
    pub attype: Vec<String>,
    pub description: String,
    pub base: String,
    pub security: String,
    #[serde(rename = "securityDefinitions")]
    pub security_definition: HashMap<String, SecurityScheme>,
    #[serde(rename = "links")]
    pub forms: Vec<Form>,
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

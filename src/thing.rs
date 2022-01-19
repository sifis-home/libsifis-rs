use std::collections::HashMap;

use serde::Deserialize;

use crate::hazard::Hazard;

type MultiLanguage = HashMap<String, String>;

#[derive(Clone, Deserialize, Debug)]
pub struct DataSchema {
    #[serde(rename = "@type")]
    pub attype: Option<String>,
    pub title: Option<String>,
    pub titles: Option<MultiLanguage>,
    pub description: Option<String>,
    pub descriptions: Option<MultiLanguage>,
    pub r#type: Option<String>,
    pub r#const: Option<serde_json::Value>,
    pub unit: Option<String>,
    #[serde(rename = "oneOf")]
    pub one_of: Option<Vec<DataSchema>>,
    pub r#enum: Option<Vec<serde_json::Value>>,
    #[serde(rename = "ReadOnly")]
    pub read_only: Option<String>,
    #[serde(rename = "WriteOnly")]
    pub write_only: Option<String>,
    pub format: Option<String>,
    pub hazards: Option<Hazard>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct IntegerSchema {
    #[serde(flatten)]
    pub data_schema: DataSchema,
    pub maximum: Option<usize>,
    pub minimum: Option<usize>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ObjectSchema {
    #[serde(flatten)]
    pub data_schema: DataSchema,
    pub properties: Option<HashMap<String, IntegerSchema>>,
    pub required: Vec<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Affordance {
    #[serde(rename = "@type")]
    pub attype: Option<String>,
    pub description: Option<String>,
    pub titles: Option<MultiLanguage>,
    #[serde(rename = "links")]
    pub forms: Vec<Form>,
    pub descriptions: Option<HashMap<String, String>>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Property {
    #[serde(flatten)]
    pub affordance: Affordance,
    // FIXME: Need to understand which subclasses
    #[serde(flatten)]
    pub schema: IntegerSchema,
    pub observable: Option<bool>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Action {
    #[serde(flatten)]
    pub affordance: Affordance,
    // FIXME: Need to understand which subclasses
    pub input: Option<ObjectSchema>,
    pub output: Option<ObjectSchema>,
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
    pub context: Vec<HashMap<String, String>>,
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

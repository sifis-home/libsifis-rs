use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::hazard::Hazard;

type MultiLanguage = HashMap<String, String>;
type DataSchemaMap = HashMap<String, SchemaType>;
type SecuritySchemeMap = HashMap<String, SecurityScheme>;

#[derive(Clone, Debug)]
pub enum OneOrMany<T>
where
    T: DeserializeOwned,
{
    One(T),
    Many(Vec<T>),
}

impl<'de, T> Deserialize<'de> for OneOrMany<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let temp = Value::deserialize(deserializer)?;
        if let Value::Array(v) = temp {
            let mut temp = Vec::with_capacity(v.len());
            for t in v {
                temp.push(serde_json::from_value(t).map_err(serde::de::Error::custom)?);
            }
            Ok(OneOrMany::Many(temp))
        } else {
            Ok(OneOrMany::One(
                serde_json::from_value(temp).map_err(serde::de::Error::custom)?,
            ))
        }
    }
}

#[derive(Clone, Debug)]
pub enum SchemaType {
    IntegerSchema(IntegerSchema),
    ObjectSchema(ObjectSchema),
}

impl<'de> Deserialize<'de> for SchemaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let temp = Value::deserialize(deserializer)?;
        if let Value::Object(v) = &temp {
            if v.contains_key("minimum") {
                Ok(SchemaType::IntegerSchema(
                    serde_json::from_value(temp).map_err(serde::de::Error::custom)?,
                ))
            } else {
                Ok(SchemaType::ObjectSchema(
                    serde_json::from_value(temp).map_err(serde::de::Error::custom)?,
                ))
            }
        } else {
            Err("Error parsing DataSchema").map_err(serde::de::Error::custom)
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct DataSchema {
    #[serde(rename = "@type")]
    pub attype: Option<String>,
    pub title: Option<String>,
    pub titles: Option<MultiLanguage>,
    pub description: Option<String>,
    pub descriptions: Option<MultiLanguage>,
    pub r#type: Option<String>,
    pub r#const: Option<Value>,
    pub unit: Option<String>,
    #[serde(rename = "oneOf")]
    pub one_of: Option<Vec<SchemaType>>,
    pub r#enum: Option<Vec<Value>>,
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
    pub properties: Option<DataSchemaMap>,
    pub required: Option<Vec<String>>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Affordance {
    #[serde(rename = "@type")]
    pub attype: Option<OneOrMany<String>>,
    pub title: Option<String>,
    pub titles: Option<MultiLanguage>,
    pub description: Option<String>,
    pub descriptions: Option<MultiLanguage>,
    #[serde(rename = "links")]
    pub forms: Vec<Form>,
    #[serde(rename = "uriVariables")]
    pub uri_variables: Option<DataSchemaMap>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Property {
    #[serde(flatten)]
    pub affordance: Affordance,
    #[serde(flatten)]
    pub schema: SchemaType,
    pub observable: Option<bool>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Action {
    #[serde(flatten)]
    pub affordance: Affordance,
    pub input: Option<SchemaType>,
    pub output: Option<SchemaType>,
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
    // FIXME Update with new security scheme
    #[serde(flatten)]
    pub scheme: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Link {
    pub href: String,
    pub rel: String,
}

// FIXME Update Form definition
#[derive(Clone, Debug, Deserialize)]
pub struct Form {
    pub href: String,
    pub rel: String,
}

/// Connected thing
#[derive(Clone, Debug, Deserialize)]
pub struct Thing {
    // FIXME Context is wrong, it is an URI or an Array
    #[serde(rename = "@context")]
    pub context: Vec<HashMap<String, String>>,
    #[serde(rename = "@type")]
    pub attype: Option<OneOrMany<String>>,
    pub title: String,
    pub description: String,
    pub base: String,
    pub links: Option<Vec<Link>>,
    pub forms: Option<Vec<Form>>,
    properties: HashMap<String, Property>,
    actions: HashMap<String, Action>,
    events: HashMap<String, Event>,
    pub security: Option<OneOrMany<String>>,
    #[serde(rename = "securityDefinitions")]
    pub security_definitions: SecuritySchemeMap,
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

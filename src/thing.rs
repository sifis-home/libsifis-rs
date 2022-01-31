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

#[derive(Clone, Debug, Deserialize)]
pub struct Form {
    pub href: String,
    #[serde(rename = "contentType", default = "default_content_type")]
    pub content_type: String,
    #[serde(rename = "contentCoding")]
    pub content_coding: Option<String>,
}

fn default_content_type() -> String {
    "application/json".to_string()
}

/// Connected thing
#[derive(Clone, Debug, Deserialize)]
pub struct Thing {
    // The context can be arbitrarily complex
    // https://www.w3.org/TR/json-ld11/#the-context
    // Let's take a value for now and assume we'll use the json-ld crate later
    #[serde(rename = "@context")]
    pub context: Value,
    #[serde(rename = "@type")]
    pub attype: Option<OneOrMany<String>>,
    pub title: String,
    pub description: Option<String>,
    pub base: Option<String>,
    pub links: Option<Vec<Link>>,
    pub forms: Option<Vec<Form>>,
    #[serde(default = "HashMap::new")]
    properties: HashMap<String, Property>,
    #[serde(default = "HashMap::new")]
    actions: HashMap<String, Action>,
    #[serde(default = "HashMap::new")]
    events: HashMap<String, Event>,
    pub security: Option<OneOrMany<String>>,
    #[serde(rename = "securityDefinitions")]
    pub security_definitions: SecuritySchemeMap,
}

impl Thing {
    pub fn properties(&self) -> impl Iterator<Item = (&String, &Property)> {
        self.properties.iter()
    }
    pub fn actions(&self) -> impl Iterator<Item = (&String, &Action)> {
        self.actions.iter()
    }
    pub fn events(&self) -> impl Iterator<Item = (&String, &Event)> {
        self.events.iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn wot_example1() {
        let ex1 = r#"
        {
            "@context": "https://www.w3.org/2019/wot/td/v1",
            "id": "urn:dev:ops:32473-WoTLamp-1234",
            "title": "MyLampThing",
            "securityDefinitions": {
                "basic_sc": {"scheme": "basic", "in":"header"}
            },
            "security": ["basic_sc"],
            "properties": {
                "status" : {
                    "type": "string",
                    "forms": [{"href": "https://mylamp.example.com/status"}]
                }
            },
            "actions": {
                "toggle" : {
                    "forms": [{"href": "https://mylamp.example.com/toggle"}]
                }
            },
            "events":{
                "overheating":{
                    "data": {"type": "string"},
                    "forms": [{
                        "href": "https://mylamp.example.com/oh",
                        "subprotocol": "longpoll"
                    }]
                }
            }
        }"#;

        let td: Thing = serde_json::from_str(ex1).unwrap();

        println!("{:?}", td);
    }
    #[test]
    fn wot_example1_no_events() {
        let ex1 = r#"
        {
            "@context": "https://www.w3.org/2019/wot/td/v1",
            "id": "urn:dev:ops:32473-WoTLamp-1234",
            "title": "MyLampThing",
            "securityDefinitions": {
                "basic_sc": {"scheme": "basic", "in":"header"}
            },
            "security": ["basic_sc"],
            "properties": {
                "status" : {
                    "type": "string",
                    "forms": [{"href": "https://mylamp.example.com/status"}]
                }
            },
            "actions": {
                "toggle" : {
                    "forms": [{"href": "https://mylamp.example.com/toggle"}]
                }
            }
        }"#;

        let td: Thing = serde_json::from_str(ex1).unwrap();

        println!("{:?}", td);
    }
}

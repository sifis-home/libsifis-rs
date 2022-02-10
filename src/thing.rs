use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use serde_with::{serde_as, OneOrMany};

type MultiLanguage = HashMap<String, String>;
type DataSchemaMap = HashMap<String, SchemaType>;
type SecuritySchemeMap = HashMap<String, SecurityScheme>;

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

/// SIFIS Hazard
///
/// Describes a possible hazard.
///
/// A risk score can *only* assume values in the range [0, 10].
/// Values outside of the defined range are invalid.
#[derive(Clone, Deserialize, Debug)]
pub struct Hazard {
    #[serde(rename = "@id")]
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "riskScore")]
    risk_score: Option<usize>,
}

impl Hazard {
    pub fn has_valid_risk_score(&self) -> bool {
        self.risk_score.map_or(false, |v| (0..11).contains(&v))
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

#[serde_as]
#[derive(Clone, Deserialize, Debug)]
pub struct Affordance {
    #[serde(rename = "@type", default = "Vec::new")]
    #[serde_as(deserialize_as = "OneOrMany<_>")]
    pub attype: Vec<String>,
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

impl Property {
    pub fn attype(&self) -> &[String] {
        &self.affordance.attype[..]
    }

    pub fn set<T: Serialize>(&self, _val: T) -> anyhow::Result<()> {
        todo!("Mockup");
    }

    pub fn get<'a, T: Deserialize<'a>>(&'a self) -> anyhow::Result<T> {
        todo!("Mockup");
    }
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
#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct Thing {
    // The context can be arbitrarily complex
    // https://www.w3.org/TR/json-ld11/#the-context
    // Let's take a value for now and assume we'll use the json-ld crate later
    #[serde(rename = "@context")]
    pub context: Value,
    #[serde(default = "String::new")]
    pub id: String,
    #[serde(rename = "@type", default = "Vec::new")]
    #[serde_as(deserialize_as = "OneOrMany<_>")]
    pub attype: Vec<String>,
    pub title: String,
    #[serde(default = "String::new")]
    pub description: String,
    #[serde(default = "String::new")]
    pub base: String,
    #[serde(default = "Vec::new")]
    pub links: Vec<Link>,
    #[serde(default = "Vec::new")]
    pub forms: Vec<Form>,
    #[serde(default = "HashMap::new")]
    pub properties: HashMap<String, Property>,
    #[serde(default = "HashMap::new")]
    pub actions: HashMap<String, Action>,
    #[serde(default = "HashMap::new")]
    pub events: HashMap<String, Event>,
    #[serde(default = "Vec::new")]
    #[serde_as(deserialize_as = "OneOrMany<_>")]
    pub security: Vec<String>,
    #[serde(rename = "securityDefinitions")]
    pub security_definitions: SecuritySchemeMap,
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
    #[test]
    fn wot_example2() {
        let ex2 = r#"
        {
            "@context": [
                "http://www.w3.org/ns/td",
                { "saref": "https://w3id.org/saref#" }
            ],
            "id": "urn:dev:ops:32473-WoTLamp-1234",
            "title": "MyLampThing",
            "@type": "saref:LightSwitch",
            "securityDefinitions": {
                "basic_sc": {"scheme": "basic", "in": "header"}
            },
            "security": "basic_sc",
            "properties": {
                "status": {
                    "@type": "saref:OnOffState",
                    "type": "string",
                    "forms": [{
                        "href": "https://mylamp.example.com/status"
                    }]
                }
            },
            "actions": {
                "toggle": {
                    "@type": "saref:ToggleCommand",
                    "forms": [{
                        "href": "https://mylamp.example.com/toggle"
                    }]
                }
            },
            "events": {
                "overheating": {
                    "data": {"type": "string"},
                    "forms": [{
                        "href": "https://mylamp.example.com/oh"
                    }]
                }
            }
        }"#;

        let td: Thing = serde_json::from_str(ex2).unwrap();

        println!("{:?}", td);
    }
    #[test]
    fn wot_example4() {
        let ex4 = r#"
        {
            "@context": "http://www.w3.org/ns/td",
            "id": "urn:dev:ops:32473-WoTLamp-1234",
            "title": "MyLampThing",
            "securityDefinitions": {
                "basic_sc": {
                    "scheme": "basic",
                    "in": "header"
                }
            },
            "security": "basic_sc",
            "properties": {
                "status": {
                    "type": "string",
                    "readOnly": false,
                    "writeOnly": false,
                    "forms": [{
                        "op": [
                            "readproperty",
                            "writeproperty"
                        ],
                        "href": "https://mylamp.example.com/status",
                        "contentType": "application/json"
                    }]
                }
            },
            "actions": {
                "toggle": {
                    "safe": false,
                    "idempotent": false,
                    "forms": [{
                        "op": "invokeaction",
                        "href": "https://mylamp.example.com/toggle",
                        "contentType": "application/json"
                    }]
                }
            },
            "events": {
                "overheating": {
                    "data": {
                        "type": "string",
                        "readOnly": false,
                        "writeOnly": false
                    },
                    "forms": [{
                        "op": "subscribeevent",
                        "href": "https://mylamp.example.com/oh",
                        "contentType": "application/json",
                        "subprotocol": "longpoll"
                    }]
                }
            }
        }"#;

        let td: Thing = serde_json::from_str(ex4).unwrap();

        println!("{:?}", td);
    }
}

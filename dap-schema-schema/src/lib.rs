use serde::Deserialize;
use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Schema {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "type")]
    pub ty: ObjectType,
    pub definitions: HashMap<String, Definition>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Definition {
    #[serde(rename_all = "camelCase")]
    AllOf {
        all_of: Vec<InterfaceOrRef>,
    },
    Interface(Interface),
    Failed {},
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum InterfaceOrRef {
    Reference {
        #[serde(rename = "$ref")]
        reference: String,
    },
    Interface(Interface),
    Failed {},
}

#[derive(Debug, Clone, Deserialize)]
pub struct Interface {
    #[serde(rename = "type")]
    ty: ObjectType,
    #[serde(default)]
    title: Option<String>,
    description: String,
    #[serde(default)]
    properties: HashMap<String, PropertyOrRef>,
    #[serde(default)]
    required: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum PropertyOrRef {
    Reference {
        #[serde(rename = "$ref")]
        reference: String,
    },
    Property(Property),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Property {
    #[serde(rename = "type")]
    ty: Type,
    description: Option<String>,
    #[serde(default, rename = "enum")]
    enumeration: Vec<String>,
}

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(try_from = "String")]
pub struct ObjectType;

impl TryFrom<String> for ObjectType {
    type Error = TypeFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "object" => Ok(Self),
            _ => Err(TypeFromStringError(value)),
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Object,
    String,
    Integer,
    Array,
}

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(try_from = "String")]
pub struct StringType;

impl TryFrom<String> for StringType {
    type Error = TypeFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "string" => Ok(Self),
            _ => Err(TypeFromStringError(value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeFromStringError(String);

impl Display for TypeFromStringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Expected 'string', got {}", self.0)
    }
}

impl Error for TypeFromStringError {}

#[cfg(test)]
mod tests {
    use super::*;

    const JSON: &'static str = include_str!("../../debugAdapterProtocol.json");

    #[test]
    fn read_schema() {
        let schema: Schema = serde_json::de::from_str(JSON).unwrap();
        println!("{schema:#?}")
    }
}

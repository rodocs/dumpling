use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Database {
    pub classes: HashMap<String, Class>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceText {
    pub text: String,
    pub source: Source,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Source {
    ApiDump,
    ReflectionMetadata,
    DevHub,
    Heuristic,
    Community,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DataType {
    Primitive(String),
    Instance(String),
    Enum(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub name: String,
    pub superclass: Option<String>,
    pub description: Option<SourceText>,
    pub properties: HashMap<String, Property>,
    pub functions: HashMap<String, Function>,
    pub events: HashMap<String, Event>,
    pub callbacks: HashMap<String, Callback>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub name: String,
    pub description: Option<SourceText>,
    pub data_type: DataType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Function {
    pub name: String,
    pub description: Option<SourceText>,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: DataType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionParameter {
    pub name: String,
    pub description: Option<SourceText>,
    pub data_type: DataType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub name: String,
    pub description: Option<SourceText>,
    pub parameters: Vec<FunctionParameter>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Callback {
    pub name: String,
    pub description: Option<SourceText>,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: DataType,
}
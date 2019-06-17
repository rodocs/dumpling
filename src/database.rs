use std::collections::HashMap;

pub struct Database {
    pub classes: HashMap<String, Class>,
}

pub struct SourceText {
    pub text: String,
    pub source: Source,
}

pub enum Source {
    ApiDump,
    ReflectionMetadata,
    DevHub,
    Heuristic,
    Community,
}

pub enum DataType {
    Primitive(String),
    Instance(String),
    Enum(String),
}

pub struct Class {
    pub name: String,
    pub superclass: Option<String>,
    pub description: Option<SourceText>,
    pub properties: HashMap<String, Property>,
    pub functions: HashMap<String, Function>,
    pub events: HashMap<String, Event>,
    pub callbacks: HashMap<String, Callback>,
}

pub struct Property {
    pub name: String,
    pub description: Option<SourceText>,
    pub data_type: DataType,
}

pub struct Function {
    pub name: String,
    pub description: Option<SourceText>,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: DataType,
}

pub struct FunctionParameter {
    pub name: String,
    pub description: Option<SourceText>,
    pub data_type: DataType,
}

pub struct Event {
    pub name: String,
    pub description: Option<SourceText>,
    pub parameters: Vec<FunctionParameter>,
}

pub struct Callback {
    pub name: String,
    pub description: Option<SourceText>,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: DataType,
}
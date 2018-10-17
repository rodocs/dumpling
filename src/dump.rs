use std::{
    collections::HashMap,
    fmt,
    fs,
    io,
    path::Path,
};

use serde_json;

#[derive(Debug)]
pub enum DumpReadError {
    InvalidJson(serde_json::Error),
    IoError(io::Error),
}

impl From<serde_json::Error> for DumpReadError {
    fn from(error: serde_json::Error) -> DumpReadError {
        DumpReadError::InvalidJson(error)
    }
}

impl From<io::Error> for DumpReadError {
    fn from(error: io::Error) -> DumpReadError {
        DumpReadError::IoError(error)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dump {
    pub classes: Vec<DumpClass>,
}

impl Dump {
    pub fn read_from_file(path: &Path) -> Result<Dump, DumpReadError> {
        let contents = fs::read_to_string(path)?;

        let dump: Dump = serde_json::from_str(&contents)?;

        Ok(dump)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ContentSource {
    ApiDump,
    ReflectionMetadata,
    Heuristic,
    Supplemental,
}

impl fmt::Display for ContentSource {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContentSource::ApiDump => write!(output, "JSON API Dump"),
            ContentSource::ReflectionMetadata => write!(output, "ReflectionMetadata.xml"),
            ContentSource::Heuristic => write!(output, "Dumpling Heuristics"),
            ContentSource::Supplemental => write!(output, "Dumpling Supplemental"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpClass {
    pub name: String,

    pub members: Vec<DumpClassMember>,

    #[serde(default = "Vec::new")]
    pub tags: Vec<String>,

    pub superclass: Option<String>,

    /// Added by Dumpling
    pub description: Option<String>,

    /// Added by Dumpling
    pub description_source: Option<ContentSource>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "MemberType")]
pub enum DumpClassMember {
    Property(DumpClassProperty),
    Function(DumpClassFunction),
    Event(DumpClassEvent),
    Callback(DumpClassCallback),
}

impl DumpClassMember {
    pub fn get_name(&self) -> &str {
        match self {
            DumpClassMember::Property(inner) => inner.name.as_str(),
            DumpClassMember::Function(inner) => inner.name.as_str(),
            DumpClassMember::Event(inner) => inner.name.as_str(),
            DumpClassMember::Callback(inner) => inner.name.as_str(),
        }
    }

    pub fn set_description(&mut self, description: String, source: ContentSource) {
        match self {
            DumpClassMember::Property(inner) => {
                inner.description = Some(description);
                inner.description_source = Some(source);
            },
            DumpClassMember::Function(inner) => {
                inner.description = Some(description);
                inner.description_source = Some(source);
            },
            DumpClassMember::Event(inner) => {
                inner.description = Some(description);
                inner.description_source = Some(source);
            },
            DumpClassMember::Callback(inner) => {
                inner.description = Some(description);
                inner.description_source = Some(source);
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpClassProperty {
    pub name: String,

    #[serde(default = "Vec::new")]
    pub tags: Vec<String>,

    #[serde(rename = "ValueType")]
    pub kind: DumpType,

    pub security: HashMap<String, String>,

    pub category: String,

    /// Added by Dumpling
    pub description: Option<String>,

    /// Added by Dumpling
    pub description_source: Option<ContentSource>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpClassFunction {
    pub name: String,

    #[serde(default = "Vec::new")]
    pub tags: Vec<String>,

    pub parameters: Vec<DumpFunctionParameter>,

    pub return_type: DumpType,

    pub security: String,

    /// Added by Dumpling
    pub description: Option<String>,

    /// Added by Dumpling
    pub description_source: Option<ContentSource>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpClassEvent {
    pub name: String,

    #[serde(default = "Vec::new")]
    pub tags: Vec<String>,

    pub parameters: Vec<DumpFunctionParameter>,

    pub security: String,

    /// Added by Dumpling
    pub description: Option<String>,

    /// Added by Dumpling
    pub description_source: Option<ContentSource>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpClassCallback {
    pub name: String,

    #[serde(default = "Vec::new")]
    pub tags: Vec<String>,

    pub security: String,

    /// Added by Dumpling
    pub description: Option<String>,

    /// Added by Dumpling
    pub description_source: Option<ContentSource>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpFunctionParameter {
    pub name: String,

    #[serde(rename = "Type")]
    pub kind: DumpType,

    /// Added by Dumpling
    pub description: Option<String>,

    /// Added by Dumpling
    pub description_source: Option<ContentSource>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpType {
    pub name: String,
    pub category: String,
}

use std::{
    fs,
    io,
    path::Path,
    fmt,
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
pub struct Dump {
    #[serde(rename = "Classes")]
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
pub struct DumpClass {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Members")]
    pub members: Vec<DumpClassMember>,

    #[serde(rename = "Tags", default = "Vec::new")]
    pub tags: Vec<String>,

    #[serde(rename = "Superclass")]
    pub superclass: Option<String>,

    /// Added by Dumpling
    #[serde(rename = "Description")]
    pub description: Option<String>,

    /// Added by Dumpling
    #[serde(rename = "DescriptionSource", default)]
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
pub struct DumpClassProperty {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Tags", default = "Vec::new")]
    pub tags: Vec<String>,

    #[serde(rename = "ValueType")]
    pub kind: DumpType,

    /// Added by Dumpling
    #[serde(rename = "Description")]
    pub description: Option<String>,

    /// Added by Dumpling
    #[serde(rename = "DescriptionSource", default)]
    pub description_source: Option<ContentSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DumpClassFunction {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Tags", default = "Vec::new")]
    pub tags: Vec<String>,

    #[serde(rename = "Parameters")]
    pub parameters: Vec<DumpFunctionParameter>,

    #[serde(rename = "ReturnType")]
    pub return_type: DumpType,

    /// Added by Dumpling
    #[serde(rename = "Description")]
    pub description: Option<String>,

    /// Added by Dumpling
    #[serde(rename = "DescriptionSource", default)]
    pub description_source: Option<ContentSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DumpClassEvent {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Tags", default = "Vec::new")]
    pub tags: Vec<String>,

    #[serde(rename = "Parameters")]
    pub parameters: Vec<DumpFunctionParameter>,

    /// Added by Dumpling
    #[serde(rename = "Description")]
    pub description: Option<String>,

    /// Added by Dumpling
    #[serde(rename = "DescriptionSource", default)]
    pub description_source: Option<ContentSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DumpClassCallback {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Tags", default = "Vec::new")]
    pub tags: Vec<String>,

    /// Added by Dumpling
    #[serde(rename = "Description")]
    pub description: Option<String>,

    /// Added by Dumpling
    #[serde(rename = "DescriptionSource", default)]
    pub description_source: Option<ContentSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DumpFunctionParameter {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Type")]
    pub kind: DumpType,

    /// Added by Dumpling
    #[serde(rename = "Description")]
    pub description: Option<String>,

    /// Added by Dumpling
    #[serde(rename = "DescriptionSource", default)]
    pub description_source: Option<ContentSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DumpType {
    #[serde(rename = "Name")]
    pub name: String,
}

use std::{
    collections::HashMap,
    fmt,
    fs,
    io,
    path::Path,
    process::Command,
};

use serde_derive::{Serialize, Deserialize};
use roblox_install::RobloxStudio;

#[derive(Debug)]
pub enum DumpReadError {
    InvalidJson(serde_json::Error),
    IoError(io::Error),
    RobloxInstall(roblox_install::Error),
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

impl From<roblox_install::Error> for DumpReadError {
    fn from(error: roblox_install::Error) -> DumpReadError {
        DumpReadError::RobloxInstall(error)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dump {
    pub classes: Vec<DumpClass>,
    pub enums: Vec<DumpEnum>,
    pub version: u32,
}

impl Dump {
    pub fn read(path: Option<&Path>) -> Result<Dump, DumpReadError> {
        match path {
            Some(path) => Dump::read_from_file(path),
            None => {
                let temp_dir = tempfile::tempdir()?;
                let dump_path = temp_dir.path().join("api-dump.json");
                let exe_path = RobloxStudio::locate()?.exe_path();

                let status = Command::new(exe_path)
                    .args(&["-API", &dump_path.display().to_string()])
                    .status()
                    .expect("Failed to spawn Roblox Studio process");

                if !status.success() {
                    panic!("Roblox Studio exited with a non-zero status code");
                }

                Dump::read_from_file(&dump_path)
            }
        }
    }

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
    DevHub,
    Heuristic,
    Supplemental,
}

impl fmt::Display for ContentSource {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContentSource::ApiDump => write!(output, "JSON API Dump"),
            ContentSource::ReflectionMetadata => write!(output, "ReflectionMetadata.xml"),
            ContentSource::DevHub => write!(output, "Roblox Developer Hub"),
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

impl DumpClass {
    pub fn properties(&self) -> impl Iterator<Item = &DumpClassProperty> {
        self.members.iter().filter_map(|member| match member {
            DumpClassMember::Property(inner) => Some(inner),
            _ => None,
        })
    }

    pub fn properties_mut(&mut self) -> impl Iterator<Item = &mut DumpClassProperty> {
        self.members.iter_mut().filter_map(|member| match member {
            DumpClassMember::Property(inner) => Some(inner),
            _ => None,
        })
    }

    pub fn has_properties(&self) -> bool {
        self.properties().next().is_some()
    }

    pub fn functions(&self) -> impl Iterator<Item = &DumpClassFunction> {
        self.members.iter().filter_map(|member| match member {
            DumpClassMember::Function(inner) => Some(inner),
            _ => None,
        })
    }

    pub fn has_functions(&self) -> bool {
        self.functions().next().is_some()
    }

    pub fn events(&self) -> impl Iterator<Item = &DumpClassEvent> {
        self.members.iter().filter_map(|member| match member {
            DumpClassMember::Event(inner) => Some(inner),
            _ => None,
        })
    }

    pub fn has_events(&self) -> bool {
        self.events().next().is_some()
    }

    pub fn callbacks(&self) -> impl Iterator<Item = &DumpClassCallback> {
        self.members.iter().filter_map(|member| match member {
            DumpClassMember::Callback(inner) => Some(inner),
            _ => None,
        })
    }

    pub fn has_callbacks(&self) -> bool {
        self.callbacks().next().is_some()
    }
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

    pub value_type: DumpType,

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpEnum {
    pub name: String,
    pub items: Vec<DumpEnumItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpEnumItem {
    pub name: String,
    pub value: u32,
}

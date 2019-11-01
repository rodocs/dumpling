use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt, fs, io,
    path::Path,
    process::Command,
};

use roblox_install::RobloxStudio;
use serde_derive::{Deserialize, Serialize};

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

    #[serde(default)]
    pub tags: BTreeSet<String>,

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
            }
            DumpClassMember::Function(inner) => {
                inner.description = Some(description);
                inner.description_source = Some(source);
            }
            DumpClassMember::Event(inner) => {
                inner.description = Some(description);
                inner.description_source = Some(source);
            }
            DumpClassMember::Callback(inner) => {
                inner.description = Some(description);
                inner.description_source = Some(source);
            }
        }
    }

    pub fn add_tag(&mut self, tag: &str) {
        let tags = match self {
            DumpClassMember::Property(inner) => &mut inner.tags,
            DumpClassMember::Function(inner) => &mut inner.tags,
            DumpClassMember::Event(inner) => &mut inner.tags,
            DumpClassMember::Callback(inner) => &mut inner.tags,
        };
        tags.insert(tag.to_owned());
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpClassProperty {
    pub name: String,

    #[serde(default)]
    pub tags: BTreeSet<String>,

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

    #[serde(default)]
    pub tags: BTreeSet<String>,

    pub parameters: Vec<DumpFunctionParameter>,

    pub return_type: DumpReturnType,

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

    #[serde(default)]
    pub tags: BTreeSet<String>,

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

    #[serde(default)]
    pub tags: BTreeSet<String>,

    pub parameters: Vec<DumpFunctionParameter>,

    pub return_type: DumpReturnType,

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
#[serde(rename_all = "PascalCase", tag = "Category", content = "Name")]
pub enum DumpType {
    Class(String),
    DataType(String),
    Enum(String),
    Group(String),
    Primitive(String),
}

impl DumpType {
    pub fn get_name(&self) -> &str {
        match self {
            DumpType::Class(name) => name,
            DumpType::DataType(name) => name,
            DumpType::Enum(name) => name,
            DumpType::Group(name) => name,
            DumpType::Primitive(name) => name,
        }
    }
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DumpReturnType {
    Single(DumpType),
    Multiple(Vec<DumpType>),
}

pub struct DumpIndexClass {
    pub class_index: usize,
    pub members: HashMap<String, usize>,
}

pub struct DumpIndex {
    classes: HashMap<String, DumpIndexClass>,
    primitives: HashSet<&'static str>,
}

pub enum DumpReference {
    Type(DumpType),
    Member(DumpType, String),
}

impl DumpIndex {
    pub fn new_from_dump(dump: &Dump) -> DumpIndex {
        DumpIndex {
            classes: dump
                .classes
                .iter()
                .enumerate()
                .map(|(index, class)| {
                    (
                        class.name.to_owned(),
                        DumpIndexClass {
                            class_index: index,
                            members: class
                                .members
                                .iter()
                                .enumerate()
                                .map(|(i, m)| (m.get_name().to_owned(), i))
                                .collect(),
                        },
                    )
                })
                .collect(),
            primitives: {
                let mut primitives = HashSet::new();
                primitives.insert("bool");
                primitives.insert("double");
                primitives.insert("float");
                primitives.insert("int");
                primitives.insert("int64");
                primitives.insert("string");
                primitives.insert("void");
                primitives
            },
        }
    }

    pub fn resolve_reference(&self, reference: &str) -> Option<DumpReference> {
        let mut split = reference.split('.');
        let type_name = split.next().unwrap();
        if type_name == "Enum" {
            // TODO: Validate enums and enum members
            if let Some(enum_name) = split.next() {
                if let Some(member_name) = split.next() {
                    Some(DumpReference::Member(
                        DumpType::Enum(enum_name.to_string()),
                        member_name.to_string(),
                    ))
                } else {
                    Some(DumpReference::Type(DumpType::Enum(enum_name.to_string())))
                }
            } else {
                None
            }
        } else if self.primitives.contains(type_name) {
            // Known primitive
            Some(DumpReference::Type(DumpType::Primitive(
                type_name.to_string(),
            )))
        } else if let Some(class_index) = self.classes.get(type_name) {
            if let Some(member_name) = split.next() {
                if class_index.members.contains_key(member_name) {
                    // Known class member
                    Some(DumpReference::Member(
                        DumpType::Class(type_name.to_string()),
                        member_name.to_string(),
                    ))
                } else {
                    // Unknown class member
                    None
                }
            } else {
                Some(DumpReference::Type(DumpType::Class(type_name.to_string())))
            }
        } else {
            // TODO: Validate DataType
            if let Some(member_name) = split.next() {
                Some(DumpReference::Member(
                    DumpType::DataType(type_name.to_string()),
                    member_name.to_string(),
                ))
            } else {
                Some(DumpReference::Type(DumpType::DataType(
                    type_name.to_string(),
                )))
            }
        }
    }
}

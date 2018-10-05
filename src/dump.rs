use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dump {
    #[serde(rename = "Classes")]
    pub classes: Vec<DumpClass>,
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
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "MemberType")]
pub enum DumpClassMember {
    Property(DumpClassProperty),
    Function(DumpClassFunction),
    Event(DumpClassEvent),
    Callback(DumpClassCallback),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DumpClassProperty {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Tags", default = "Vec::new")]
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DumpClassFunction {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Tags", default = "Vec::new")]
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DumpClassEvent {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Tags", default = "Vec::new")]
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DumpClassCallback {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Tags", default = "Vec::new")]
    pub tags: Vec<String>,
}

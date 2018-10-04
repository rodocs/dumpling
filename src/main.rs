use std::{
    fmt::{self, Write},
};

use serde_derive::{Serialize, Deserialize};
use serde_json;

static DUMP_SOURCE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/dump.json"));

#[derive(Debug, Serialize, Deserialize)]
struct Dump {
    #[serde(rename = "Classes")]
    classes: Vec<DumpClass>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DumpClass {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Members")]
    members: Vec<DumpClassMember>,

    #[serde(rename = "Tags", default = "Vec::new")]
    tags: Vec<String>,

    #[serde(rename = "Superclass")]
    superclass: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "MemberType")]
enum DumpClassMember {
    Property(DumpClassProperty),
    Function(DumpClassFunction),
    Event(DumpClassEvent),
    Callback(DumpClassCallback),
}

#[derive(Debug, Serialize, Deserialize)]
struct DumpClassProperty {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Tags", default = "Vec::new")]
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DumpClassFunction {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Tags", default = "Vec::new")]
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DumpClassEvent {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Tags", default = "Vec::new")]
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DumpClassCallback {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Tags", default = "Vec::new")]
    tags: Vec<String>,
}

fn emit_dump(dump: &Dump, output: &mut String) -> fmt::Result {
    writeln!(output, "<!doctype html>")?;
    writeln!(output, "<html>")?;
    writeln!(output, "<head><title>RoDumpster</title></head")?;
    writeln!(output, "<body>")?;

    for class in &dump.classes {
        emit_class(class, output)?;
    }

    writeln!(output, "</body>")?;
    writeln!(output, "</html>")
}

fn emit_class(class: &DumpClass, output: &mut String) -> fmt::Result {
    writeln!(output, "<h1>{}</h1>", class.name)?;

    match &class.superclass {
        Some(superclass) => writeln!(output, "<p>Inherits: {}</p>", superclass)?,
        None => {},
    }

    if class.tags.len() > 0 {
        writeln!(output, "<p>Tags: {}</p>", class.tags.join(", "))?;
    }

    writeln!(output, "<ul>")?;

    for member in &class.members {
        emit_member(member, output)?
    }

    writeln!(output, "</ul>")
}

fn emit_member(member: &DumpClassMember, output: &mut String) -> fmt::Result {
    writeln!(output, "<li>{:?}</li>", member)
}

fn main() {
    let dump: Dump = serde_json::from_str(DUMP_SOURCE).unwrap();

    let mut output = String::new();

    emit_dump(&dump, &mut output).unwrap();

    println!("{}", output);
}

mod dump;

use std::{
    fmt::{self, Write},
};

use serde_json;

use crate::dump::{Dump, DumpClass, DumpClassMember};

static DUMP_SOURCE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/dump.json"));

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

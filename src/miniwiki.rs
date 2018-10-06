use std::{
    fmt::{self, Write},
};

use crate::{
    dump::{Dump, DumpClass, DumpClassMember, DumpClassProperty, DumpClassFunction},
    templating::{HtmlEscape, tag},
};

static STYLE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/miniwiki.css"));

pub fn emit_dump(dump: &Dump, output: &mut String) -> fmt::Result {
    writeln!(output, "<!doctype html>")?;
    writeln!(output, "<html>")?;

    writeln!(output, "{}", tag("head")
        .append(tag("title").append("Rodumpster"))
        .append(tag("style").append(STYLE))
    )?;
    writeln!(output, "<body>")?;

    writeln!(output, r#"<div class="dump-classes">"#)?;
    for class in &dump.classes {
        emit_class(class, output)?;
    }
    writeln!(output, "</div>")?;

    writeln!(output, "</body>")?;
    writeln!(output, "</html>")?;

    Ok(())
}

fn emit_class(class: &DumpClass, output: &mut String) -> fmt::Result {
    writeln!(output, r#"<div class="dump-class">"#)?;
    writeln!(output, r#"<div class="dump-class-title">{}</div>"#, HtmlEscape(&class.name))?;

    match &class.superclass {
        Some(superclass) => writeln!(output, "<p>Inherits: {}</p>", HtmlEscape(superclass))?,
        None => {},
    }

    if class.tags.len() > 0 {
        writeln!(output, "<p>Tags: {}</p>", HtmlEscape(&class.tags.join(", ")))?;
    }

    match &class.description {
        Some(description) => writeln!(output, "<p>{}</p>", HtmlEscape(description))?,
        None =>  {},
    }

    let mut properties = Vec::new();
    let mut functions = Vec::new();
    let mut events = Vec::new();
    let mut callbacks = Vec::new();

    for member in &class.members {
        match member {
            DumpClassMember::Property(property) => properties.push(property),
            DumpClassMember::Function(function) => functions.push(function),
            DumpClassMember::Event(event) => events.push(event),
            DumpClassMember::Callback(callback) => callbacks.push(callback),
        }
    }

    if properties.len() > 0 {
        writeln!(output, r#"<div class="dump-class-subtitle">Properties</div>"#)?;
        writeln!(output, r#"<div class="dump-class-properties">"#)?;
        for property in &properties {
            emit_property(property, output)?;
        }
        writeln!(output, "</div>")?;
    }

    if functions.len() > 0 {
        writeln!(output, r#"<div class="dump-class-subtitle">Functions</div>"#)?;
        writeln!(output, r#"<div class="dump-class-functions">"#)?;
        for function in &functions {
            emit_function(function, output)?;
        }
        writeln!(output, "</div>")?;
    }

    if events.len() > 0 {
        writeln!(output, r#"<div class="dump-class-subtitle">Events</div>"#)?;
        writeln!(output, r#"<div class="dump-class-events">"#)?;
        for event in &events {
            // emit_event(event, output)?;
        }
        writeln!(output, "</div>")?;
    }

    if callbacks.len() > 0 {
        writeln!(output, r#"<div class="dump-class-subtitle">Callbacks</div>"#)?;
        writeln!(output, r#"<div class="dump-class-callbacks">"#)?;
        for callback in &callbacks {
            // emit_callback(callback, output)?;
        }
        writeln!(output, "</div>")?;
    }

    writeln!(output, "</div>")?;

    Ok(())
}

fn emit_property(property: &DumpClassProperty, output: &mut String) -> fmt::Result {
    writeln!(output, r#"<div class="dump-class-property">"#)?;
    writeln!(output, r#"<div class="dump-class-property-name">{}</div>"#, HtmlEscape(&property.name))?;

    if let Some(description) = &property.description {
        writeln!(output, "{}", HtmlEscape(description))?;
    }

    writeln!(output, "</div>")?;

    Ok(())
}

fn emit_function(function: &DumpClassFunction, output: &mut String) -> fmt::Result {
    writeln!(output, r#"<div class="dump-class-function">"#)?;
    writeln!(output, r#"<div class="dump-class-function-name">{}</div>"#, HtmlEscape(&function.name))?;

    if let Some(description) = &function.description {
        writeln!(output, "{}", HtmlEscape(description))?;
    }

    writeln!(output, "</div>")?;

    Ok(())
}
use std::{
    fmt::{self, Write},
};

use pulldown_cmark;

use ::{
    dump::{
        Dump,
        DumpClass,
        DumpClassCallback,
        DumpClassEvent,
        DumpClassFunction,
        DumpClassMember,
        DumpClassProperty,
    },
    templating::{HtmlTag, HtmlContent, tag, tag_class},
};

static STYLE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/miniwiki.css"));

fn markdownify(input: &str) -> HtmlContent {
    let parser = pulldown_cmark::Parser::new(input);
    let mut output = String::new();
    pulldown_cmark::html::push_html(&mut output, parser);

    HtmlContent::Raw(output)
}

pub fn emit_wiki(dump: &Dump, output: &mut String) -> fmt::Result {
    writeln!(output, "<!doctype html>")?;

    let html = tag("html")
        .child(tag("head")
            .child(tag("title").child("Dumpling"))
            .child(tag("style").child(STYLE)))
        .child(tag("body")
            .child(tag_class("div", "dump-classes")
                .children(dump.classes.iter().map(emit_class))));

    write!(output, "{}", html)
}

fn emit_class(class: &DumpClass) -> HtmlTag {
    let mut container = tag_class("div", "dump-class")
        .child(tag_class("a", "dump-class-title")
            .attr("id", &class.name)
            .attr("href", &format!("#{}", class.name))
            .child(&class.name));

    if let Some(superclass) = &class.superclass {
        container.add_child(tag_class("p", "dump-class-inherits")
            .child("Inherits: ")
            .child(emit_type_link(superclass)));
    }

    if class.tags.len() > 0 {
        container.add_child(tag_class("p", "dump-class-tags")
            .child("Tags: ")
            .child(&class.tags.join(", ")));
    }

    if let Some(description) = &class.description {
        container.add_child(tag_class("div", "dump-class-description markdown")
            .child(markdownify(description)));
    }

    let mut properties = tag_class("div", "dump-class-properties");
    let mut functions = tag_class("div", "dump-class-functions");
    let mut events = tag_class("div", "dump-class-events");
    let mut callbacks = tag_class("div", "dump-class-callbacks");

    for member in &class.members {
        match member {
            DumpClassMember::Property(property) => properties.add_child(emit_property(property)),
            DumpClassMember::Function(function) => functions.add_child(emit_function(function)),
            DumpClassMember::Event(event) => events.add_child(emit_event(event)),
            DumpClassMember::Callback(callback) => callbacks.add_child(emit_callback(callback)),
        }
    }

    if properties.child_count() > 0 {
        container.add_child(tag_class("div", "dump-class-subtitle").child("Properties"));
        container.add_child(properties);
    }

    if functions.child_count() > 0 {
        container.add_child(tag_class("div", "dump-class-subtitle").child("Functions"));
        container.add_child(functions);
    }

    if events.child_count() > 0 {
        container.add_child(tag_class("div", "dump-class-subtitle").child("Events"));
        container.add_child(events);
    }

    if callbacks.child_count() > 0 {
        container.add_child(tag_class("div", "dump-class-subtitle").child("Callbacks"));
        container.add_child(callbacks);
    }

    container
}

fn emit_property(property: &DumpClassProperty) -> HtmlTag {
    let description = property.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or("*No description available.*");

    tag_class("div", "dump-class-property")
        .child(tag("span")
            .child(tag_class("span", "dump-class-property-name")
                .child(&property.name))
            .child(": ")
            .child(emit_type_link(&property.kind.name)))
        .child(tag_class("div", "dump-class-property-description markdown")
            .child(markdownify(description)))
}

fn emit_function(function: &DumpClassFunction) -> HtmlTag {
    let description = function.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or("*No description available.*");

    let parameters = function.parameters
        .iter()
        .enumerate()
        .map(|(index, param)| {
            let mut parameter = tag_class("div", "dump-function-argument")
                .child(tag_class("span", "dump-function-argument-name").child(&param.name))
                .child(": ")
                .child(emit_type_link(&param.kind.name));

            if index < function.parameters.len() - 1 {
                parameter.add_child(", ");
            }

            parameter
        });

    tag_class("div", "dump-class-function")
        .child(tag_class("div", "dump-class-function-signature")
            .child(tag_class("span", "dump-class-function-name").child(&function.name))
            .child("(")
            .children(parameters)
            .child("): ")
            .child(emit_type_link(&function.return_type.name)))
        .child(tag_class("div", "dump-class-function-description markdown")
            .child(markdownify(description)))
}

fn emit_event(event: &DumpClassEvent) -> HtmlTag {
    let description = event.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or("*No description available.*");

    tag_class("div", "dump-class-event")
        .child(tag_class("div", "dump-class-event-name")
            .child(&event.name))
        .child(tag_class("div", "dump-class-event-description markdown")
                .child(markdownify(description)))
}

fn emit_callback(callback: &DumpClassCallback) -> HtmlTag {
    let description = callback.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or("*No description available.*");

    tag_class("div", "dump-class-callback")
        .child(tag_class("div", "dump-class-callback-name")
            .child(&callback.name))
        .child(tag_class("div", "dump-class-callback-description markdown")
            .child(markdownify(description)))
}

fn emit_type_link(name: &str) -> HtmlTag {
    tag("a")
        .attr("href", &format!("#{}", name))
        .child(name)
}
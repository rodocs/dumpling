use std::{
    fmt::{self, Write},
};

use pulldown_cmark;

use crate::{
    dump::{
        ContentSource,
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

static DEFAULT_DESCRIPTION: &str = "*No description available.*";

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
            // Templating system doesn't do self-closing tags yet
            .child(HtmlContent::Raw("<meta charset=\"utf-8\" />".to_string()))
            .child(HtmlContent::Raw("<meta name=\"viewport\" content=\"width=device-width,initial-scale=1,shrink-to-fit=no\" />".to_string()))
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

    let description = class.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    container.add_child(tag_class("div", "dump-class-description")
        .child(tag_class("div", "dump-class-description-text markdown")
            .child(markdownify(description)))
        .child(tag_class("div", "dump-class-description-meta")
            .child(class.description_source.map(|source| {
                tag_class("span", "dump-info")
                    .attr("title", &format!("Content source: {}", source))
            }))));

    let mut properties = tag_class("div", "dump-class-member-section-list");
    let mut functions = tag_class("div", "dump-class-member-section-list");
    let mut events = tag_class("div", "dump-class-member-section-list");
    let mut callbacks = tag_class("div", "dump-class-member-section-list");

    for member in &class.members {
        match member {
            DumpClassMember::Property(property) => properties.add_child(emit_property(property)),
            DumpClassMember::Function(function) => functions.add_child(emit_function(function)),
            DumpClassMember::Event(event) => events.add_child(emit_event(event)),
            DumpClassMember::Callback(callback) => callbacks.add_child(emit_callback(callback)),
        }
    }

    if properties.child_count() > 0 {
        container.add_child(tag_class("div", "dump-class-member-section")
            .child(tag_class("div", "dump-class-subtitle").child("Properties"))
            .child(properties));
    }

    if functions.child_count() > 0 {
        container.add_child(tag_class("div", "dump-class-member-section")
            .child(tag_class("div", "dump-class-subtitle").child("Functions"))
            .child(functions));
    }

    if events.child_count() > 0 {
        container.add_child(tag_class("div", "dump-class-member-section")
            .child(tag_class("div", "dump-class-subtitle").child("Events"))
            .child(events));
    }

    if callbacks.child_count() > 0 {
        container.add_child(tag_class("div", "dump-class-member-section")
            .child(tag_class("div", "dump-class-subtitle").child("Callbacks"))
            .child(callbacks));
    }

    container
}

fn emit_property(property: &DumpClassProperty) -> HtmlTag {
    let description = property.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    tag_class("div", "dump-class-member")
        .child(tag("span")
            .child(tag_class("span", "dump-class-member-name")
                .child(&property.name))
            .child(": ")
            .child(emit_type_link(&property.value_type.name)))
        .child(emit_member_description(description, property.description_source))
}

fn emit_function(function: &DumpClassFunction) -> HtmlTag {
    let description = function.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    let parameters = function.parameters
        .iter()
        .enumerate()
        .map(|(index, param)| {
            let mut parameter = tag_class("div", "dump-function-argument")
                .child(&param.name)
                .child(": ")
                .child(emit_type_link(&param.kind.name));

            if index < function.parameters.len() - 1 {
                parameter.add_child(", ");
            }

            parameter
        });

    tag_class("div", "dump-class-member")
        .child(tag_class("div", "dump-class-function-signature")
            .child(tag_class("span", "dump-class-member-name").child(&function.name))
            .child("(")
            .children(parameters)
            .child("): ")
            .child(emit_type_link(&function.return_type.name)))
        .child(emit_member_description(description, function.description_source))
}

fn emit_event(event: &DumpClassEvent) -> HtmlTag {
    let description = event.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    tag_class("div", "dump-class-member")
        .child(tag_class("div", "dump-class-member-name")
            .child(&event.name))
        .child(emit_member_description(description, event.description_source))
}

fn emit_callback(callback: &DumpClassCallback) -> HtmlTag {
    let description = callback.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    tag_class("div", "dump-class-member")
        .child(tag_class("div", "dump-class-member-name")
            .child(&callback.name))
        .child(emit_member_description(description, callback.description_source))
}

fn emit_member_description(description: &str, source: Option<ContentSource>) -> HtmlTag {
    tag_class("div", "dump-class-member-description")
        .child(tag_class("div", "dump-class-member-description-text markdown")
            .child(markdownify(description)))
        .child(tag_class("div", "dump-class-member-meta")
            .child(source.map(|source| {
                tag_class("span", "dump-info")
                    .attr("title", &format!("Content source: {}", source))
            })))

}

fn emit_type_link(name: &str) -> HtmlTag {
    tag("a")
        .attr("href", &format!("#{}", name))
        .child(name)
}
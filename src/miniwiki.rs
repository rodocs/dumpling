use std::{
    fmt::{self, Write},
};

use pulldown_cmark;
use snax::{snax, UnescapedText, HtmlContent as SnaxHtmlContent, Fragment};

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

fn render_markdown(input: &str) -> SnaxHtmlContent {
    let parser = pulldown_cmark::Parser::new(input);
    let mut output = String::new();
    pulldown_cmark::html::push_html(&mut output, parser);

    UnescapedText::new(output).into()
}

fn markdownify(input: &str) -> HtmlContent {
    let parser = pulldown_cmark::Parser::new(input);
    let mut output = String::new();
    pulldown_cmark::html::push_html(&mut output, parser);

    HtmlContent::Raw(output)
}

pub fn emit_wiki(dump: &Dump, output: &mut String) -> fmt::Result {
    writeln!(output, "<!doctype html>")?;

    let html = snax!(
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width,initial-scale=1,shrink-to-fit=no" />
                <style>{ UnescapedText::new(STYLE) }</style>
            </head>
            <body>
                <div class="dump-classes">
                    { Fragment::new(dump.classes.iter().map(emit_class).map(|v| UnescapedText::new(v.to_string()))) }
                </div>
            </body>
        </html>
    );

    write!(output, "{}", html)
}

fn render_class(class: &DumpClass) -> SnaxHtmlContent {
    snax!(
        <div class="dump-class">
            <a class="dump-class-title" id={ class.name.to_owned() } href={ format!("#{}", class.name) }>
                { class.name.to_owned() }
            </a>
            { class.superclass.as_ref().map(|superclass| snax!(
                <p class="dump-class-inherits">
                    "Inherits: "
                    { render_type_link(&superclass) }
                </p>
            )) }
        </div>
    )
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
            DumpClassMember::Property(property) => properties.add_child(emit_property(property, &class.name)),
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

fn render_property(property: &DumpClassProperty, parent_name: &str) -> SnaxHtmlContent {
    let description = property.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    let qualified_name = format!("{}.{}", parent_name, property.name);

    snax!(
        <div class="dump-class-member" id={ qualified_name.clone() }>
            <a class="dump-class-member-anchor" href={ format!("#{}", qualified_name) }>
                "#"
            </a>
            <span>
                <span class="dump-class-member-name">{ property.name.to_owned() }</span>
                ": "
                { render_type_link(&property.value_type.name) }
            </span>
            { render_member_description(description, property.description_source) }
        </div>
    )
}

fn emit_property(property: &DumpClassProperty, parent_name: &str) -> HtmlContent {
    HtmlContent::Raw(render_property(property, parent_name).to_string())
}

fn render_function(function: &DumpClassFunction) -> SnaxHtmlContent {
    let description = function.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    let parameters = function.parameters
        .iter()
        .enumerate()
        .map(|(index, param)| snax!(
            <div class="dump-function-argument">
                { param.name.to_owned() }
                ": "
                { render_type_link(&param.kind.name) }
                {
                    if index < function.parameters.len() - 1 {
                        ",".into()
                    } else {
                        SnaxHtmlContent::None
                    }
                }
            </div>
        ));

    snax!(
        <div class="dump-class-member">
            <div class="dump-class-function-signature">
                <span class="dump-class-member-name">
                    { function.name.to_owned() }
                </span>
                "("
                { Fragment::new(parameters) }
                "): "
                { render_type_link(&function.return_type.name) }
            </div>
            { render_member_description(description, function.description_source) }
        </div>
    )
}

fn emit_function(function: &DumpClassFunction) -> HtmlContent {
    HtmlContent::Raw(render_function(function).to_string())
}

fn render_event(event: &DumpClassEvent) -> SnaxHtmlContent {
    let description = event.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    snax!(
        <div class="dump-class-member">
            <div class="dump-class-member-name">
                { event.name.to_owned() }
            </div>
            { render_member_description(description, event.description_source) }
        </div>
    )
}

fn emit_event(event: &DumpClassEvent) -> HtmlContent {
    HtmlContent::Raw(render_event(event).to_string())
}

fn render_callback(callback: &DumpClassCallback) -> SnaxHtmlContent {
    let description = callback.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    snax!(
        <div class="dump-class-member">
            <div class="dump-class-member-name">
                { callback.name.to_owned() }
            </div>
            { render_member_description(description, callback.description_source) }
        </div>
    )
}

fn emit_callback(callback: &DumpClassCallback) -> HtmlContent {
    HtmlContent::Raw(render_callback(callback).to_string())
}

fn render_member_description(description: &str, source: Option<ContentSource>) -> SnaxHtmlContent {
    snax!(
        <div class="dump-class-member-description">
            <div class="dump-class-member-description-text markdown">
                { render_markdown(description) }
            </div>
            <div class="dump-class-member-meta">
                { Fragment::new(source.map(|source| snax!(
                    <span class="dump-info" title={ format!("Content source: {}", source) } />
                ))) }
            </div>
        </div>
    )
}

fn emit_member_description(description: &str, source: Option<ContentSource>) -> HtmlContent {
    HtmlContent::Raw(render_member_description(description, source).to_string())
}

fn render_type_link(name: &str) -> SnaxHtmlContent {
    snax!(
        <a href={ format!("#{}", name) }>
            { name.to_string() }
        </a>
    )
}

fn emit_type_link(name: &str) -> HtmlContent {
    HtmlContent::Raw(render_type_link(name).to_string())
}
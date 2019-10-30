use std::{
    collections::BTreeSet,
    fmt::{self, Write},
};

use pulldown_cmark;
use ritz::{html, UnescapedText, HtmlContent, Fragment};

use crate::{
    dump::{
        ContentSource,
        Dump,
        DumpClass,
        DumpClassCallback,
        DumpClassEvent,
        DumpClassFunction,
        DumpClassProperty,
        DumpFunctionParameter
    },
};

static STYLE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/miniwiki.css"));

static DEFAULT_DESCRIPTION: &str = "*No description available.*";

fn render_markdown(input: &str) -> HtmlContent {
    let parser = pulldown_cmark::Parser::new(input);
    let mut output = String::new();
    pulldown_cmark::html::push_html(&mut output, parser);

    UnescapedText::new(output).into()
}

pub fn emit_wiki(dump: &Dump, output: &mut String) -> fmt::Result {
    writeln!(output, "<!doctype html>")?;

    let html = html!(
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width,initial-scale=1,shrink-to-fit=no" />
                <style>{ UnescapedText::new(STYLE) }</style>
                <title>"Rodocs Mini"</title>
            </head>
            <body>
                <div class="dump-classes">
                    { Fragment::new(dump.classes.iter().map(render_class)) }
                </div>
            </body>
        </html>
    );

    write!(output, "{}", html)
}

fn render_class(class: &DumpClass) -> HtmlContent {
    let description = class.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    let mut element_class = "dump-class".to_owned();
    if class.tags.contains("Deprecated")
    {
        element_class.push_str(" dump-class-deprecated");
    }

    html!(
        <div id={ &class.name } class={ element_class }>
            <a class="dump-class-title" href={ format!("#{}", class.name) }>
                { &class.name }
            </a>

            { class.superclass.as_ref().map(|superclass| html!(
                <p class="dump-class-inherits">
                    "Inherits: "
                    { render_type_link(&superclass) }
                </p>
            )) }

            { if class.tags.len() > 0 {
                html!(
                     <p class="dump-class-tags">
                        "Tags: "
                        { class.tags.iter().map(|value| value.as_str()).collect::<Vec<_>>().join(", ")}
                     </p>
                )
            } else {
                HtmlContent::None
            }}

            <div class="dump-class-description">
                <div class="dump-class-description-text markdown">
                    { render_markdown(description) }
                </div>
                <div class="dump-class-description-meta">
                    { class.description_source.map(|source| html!(
                        <span class="dump-info" title={ format!("Content source: {}", source) } />
                    )) }
                </div>
            </div>

            { if class.has_properties() {
                html!(
                    <div class="dump-class-member-section">
                        <div class="dump-class-subtitle">"Properties"</div>
                        <div class="dump-class-member-section-list">
                            { Fragment::new(class.properties().map(|property| render_property(property, &class.name))) }
                        </div>
                    </div>
                )
            } else {
                HtmlContent::None
            } }

            { if class.has_functions() {
                html!(
                    <div class="dump-class-member-section">
                        <div class="dump-class-subtitle">"Functions"</div>
                        <div class="dump-class-member-section-list">
                            { Fragment::new(class.functions().map(|function| render_function(function, &class.name))) }
                        </div>
                    </div>
                )
            } else {
                HtmlContent::None
            } }

            { if class.has_events() {
                html!(
                    <div class="dump-class-member-section">
                        <div class="dump-class-subtitle">"Events"</div>
                        <div class="dump-class-member-section-list">
                            { Fragment::new(class.events().map(|event| render_event(event, &class.name))) }
                        </div>
                    </div>
                )
            } else {
                HtmlContent::None
            } }

            { if class.has_callbacks() {
                html!(
                    <div class="dump-class-member-section">
                        <div class="dump-class-subtitle">"Callbacks"</div>
                        <div class="dump-class-member-section-list">
                            { Fragment::new(class.callbacks().map(|callback| render_callback(callback, &class.name))) }
                        </div>
                    </div>
                )
            } else {
                HtmlContent::None
            } }
        </div>
    )
}

fn render_property<'a>(property: &'a DumpClassProperty, parent_name: &str) -> HtmlContent<'a> {
    let description = property.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    let qualified_name = format!("{}.{}", parent_name, property.name);

    html!(
        <div class={ member_element_class(&property.tags, "dump-class-property") } id={ qualified_name.clone() }>
            <div class="dump-class-property-signature">
                <a class="dump-class-member-name" href={ format!("#{}", qualified_name) }>
                    { &property.name }
                </a>
                ": "
                { render_type_link(&property.value_type.name) }
            </div>
            { render_member_description(description, property.description_source) }
        </div>
    )
}

fn render_function<'a>(function: &'a DumpClassFunction, parent_name: &str) -> HtmlContent<'a> {
    let description = function.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    let qualified_name = format!("{}.{}", parent_name, function.name);

    html!(
        <div class={ member_element_class(&function.tags, "dump-class-function") } id={ qualified_name.clone() }>
            <div class="dump-function-signature">
                <a class="dump-class-member-name" href={ format!("#{}", qualified_name) }>
                    { &function.name }
                </a>
                "("
                { render_arguments(&function.parameters) }
                "): "
                { render_type_link(&function.return_type.name) }
            </div>
            { render_member_description(description, function.description_source) }
        </div>
    )
}

fn render_event<'a>(event: &'a DumpClassEvent, parent_name: &str) -> HtmlContent<'a> {
    let description = event.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    let qualified_name = format!("{}.{}", parent_name, event.name);

    html!(
        <div class={ member_element_class(&event.tags, "dump-class-event") } id={ qualified_name.clone() }>
            <div class="dump-function-signature">
                <a class="dump-class-member-name" href={ format!("#{}", qualified_name)}>
                    { &event.name }
                </a>
                ": "
                { render_type_link("RBXScriptSignal") }
                "("
                { render_arguments(&event.parameters) }
                ")"
            </div>
            { render_member_description(description, event.description_source) }
        </div>
    )
}

fn render_callback<'a>(callback: &'a DumpClassCallback, parent_name: &str) -> HtmlContent<'a> {
    let description = callback.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    let qualified_name = format!("{}.{}", parent_name, callback.name);

    html!(
        <div class={ member_element_class(&callback.tags, "dump-class-callback") } id={ qualified_name.clone() }>
            <div class="dump-function-signature">
                <a class="dump-class-member-name" href={ format!("#{}", qualified_name)}>
                    { &callback.name }
                </a>
                ": function("
                { render_arguments(&callback.parameters) }
                "): "
                { render_type_link(&callback.return_type.name) }
            </div>
            { render_member_description(description, callback.description_source) }
        </div>
    )
}

fn render_member_description(description: &str, source: Option<ContentSource>) -> HtmlContent {
    html!(
        <div class="dump-class-member-description">
            <div class="dump-class-member-description-text markdown">
                { render_markdown(description) }
            </div>
            <div class="dump-class-member-meta">
                { Fragment::new(source.map(|source| html!(
                    <span class="dump-info" title={ format!("Content source: {}", source) } />
                ))) }
            </div>
        </div>
    )
}

fn render_type_link(name: &str) -> HtmlContent {
    html!(
        <a href={ format!("#{}", name) }>
            { name }
        </a>
    )
}

fn render_arguments(parameters: &Vec<DumpFunctionParameter>) -> Fragment {
    Fragment::new(parameters
        .iter()
        .enumerate()
        .map(|(index, param)| html!(
            <div class="dump-function-argument">
                { &param.name }
                ": "
                { render_type_link(&param.kind.name) }
                {
                    if index < parameters.len() - 1 {
                        ",".into()
                    } else {
                        HtmlContent::None
                    }
                }
            </div>
        )))
}

fn member_element_class(tags: &BTreeSet<String>, main_class: &str) -> String {
    let mut element_class = "dump-class-member ".to_owned();
    element_class.push_str(main_class);
    if tags.contains("Deprecated")
    {
        element_class.push_str(" dump-member-deprecated");
    }
    return element_class;
}

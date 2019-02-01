use std::{
    fmt::{self, Write},
};

use pulldown_cmark;
use snax::{snax, UnescapedText, HtmlContent, Fragment};

use crate::{
    dump::{
        ContentSource,
        Dump,
        DumpClass,
        DumpClassCallback,
        DumpClassEvent,
        DumpClassFunction,
        DumpClassProperty,
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

    let html = snax!(
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width,initial-scale=1,shrink-to-fit=no" />
                <style>{ UnescapedText::new(STYLE) }</style>
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

    snax!(
        <div class="dump-class">
            <a class="dump-class-title" id={ &class.name } href={ format!("#{}", class.name) }>
                { &class.name }
            </a>

            { class.superclass.as_ref().map(|superclass| snax!(
                <p class="dump-class-inherits">
                    "Inherits: "
                    { render_type_link(&superclass) }
                </p>
            )) }

            { if class.tags.len() > 0 {
                snax!(
                     <p class="dump-class-tags">
                        "Tags: "
                        { class.tags.join(", ") }
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
                    { class.description_source.map(|source| snax!(
                        <span class="dump-info" title={ format!("Content source: {}", source) } />
                    )) }
                </div>
            </div>

            { if class.has_properties() {
                snax!(
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
                snax!(
                    <div class="dump-class-member-section">
                        <div class="dump-class-subtitle">"Functions"</div>
                        <div class="dump-class-member-section-list">
                            { Fragment::new(class.functions().map(render_function)) }
                        </div>
                    </div>
                )
            } else {
                HtmlContent::None
            } }

            { if class.has_events() {
                snax!(
                    <div class="dump-class-member-section">
                        <div class="dump-class-subtitle">"Events"</div>
                        <div class="dump-class-member-section-list">
                            { Fragment::new(class.events().map(render_event)) }
                        </div>
                    </div>
                )
            } else {
                HtmlContent::None
            } }

            { if class.has_callbacks() {
                snax!(
                    <div class="dump-class-member-section">
                        <div class="dump-class-subtitle">"Callbacks"</div>
                        <div class="dump-class-member-section-list">
                            { Fragment::new(class.callbacks().map(render_callback)) }
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

    snax!(
        <div class="dump-class-member" id={ qualified_name.clone() }>
            <a class="dump-class-member-anchor" href={ format!("#{}", qualified_name) }>
                "#"
            </a>
            <span>
                <span class="dump-class-member-name">{ &property.name }</span>
                ": "
                { render_type_link(&property.value_type.name) }
            </span>
            { render_member_description(description, property.description_source) }
        </div>
    )
}

fn render_function(function: &DumpClassFunction) -> HtmlContent {
    let description = function.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    let parameters = function.parameters
        .iter()
        .enumerate()
        .map(|(index, param)| snax!(
            <div class="dump-function-argument">
                { &param.name }
                ": "
                { render_type_link(&param.kind.name) }
                {
                    if index < function.parameters.len() - 1 {
                        ",".into()
                    } else {
                        HtmlContent::None
                    }
                }
            </div>
        ));

    snax!(
        <div class="dump-class-member">
            <div class="dump-class-function-signature">
                <span class="dump-class-member-name">
                    { &function.name }
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

fn render_event(event: &DumpClassEvent) -> HtmlContent {
    let description = event.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    snax!(
        <div class="dump-class-member">
            <div class="dump-class-member-name">
                { &event.name }
            </div>
            { render_member_description(description, event.description_source) }
        </div>
    )
}

fn render_callback(callback: &DumpClassCallback) -> HtmlContent {
    let description = callback.description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    snax!(
        <div class="dump-class-member">
            <div class="dump-class-member-name">
                { &callback.name }
            </div>
            { render_member_description(description, callback.description_source) }
        </div>
    )
}

fn render_member_description(description: &str, source: Option<ContentSource>) -> HtmlContent {
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

fn render_type_link(name: &str) -> HtmlContent {
    snax!(
        <a href={ format!("#{}", name) }>
            { name }
        </a>
    )
}
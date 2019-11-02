use std::{
    collections::BTreeSet,
    fmt::{self, Write},
};

use pulldown_cmark;
use ritz::{html, Fragment, HtmlContent, UnescapedText};

use crate::dump::{
    ContentSource, Dump, DumpClass, DumpClassCallback, DumpClassEvent, DumpClassFunction,
    DumpClassProperty, DumpFunctionParameter, DumpIndex, DumpReference, DumpReturnType, DumpType,
};

static STYLE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/resources/miniwiki.css"
));

static DEFAULT_DESCRIPTION: &str = "*No description available.*";

fn render_markdown(input: &str, dump_index: &DumpIndex) -> HtmlContent<'static> {
    let callback = |_norm: &str, raw: &str| match dump_index.resolve_reference(raw)? {
        DumpReference::Type(dump_type) => {
            let type_link = format!("#{}", dump_type.get_name());
            Some((type_link.clone(), type_link))
        }
        DumpReference::Member(dump_type, member_name) => {
            let type_link = format!("#{}.{}", dump_type.get_name(), member_name);
            Some((type_link.clone(), type_link))
        }
    };

    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_TABLES);

    let parser =
        pulldown_cmark::Parser::new_with_broken_link_callback(input, options, Some(&callback));

    let mut output = String::new();
    pulldown_cmark::html::push_html(&mut output, parser);

    UnescapedText::new(output).into()
}

pub fn emit_wiki(dump: &Dump, output: &mut String) -> fmt::Result {
    writeln!(output, "<!doctype html>")?;

    let dump_index = DumpIndex::new_from_dump(dump);
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
                    { Fragment::new(dump.classes.iter().map(|class| render_class(class, &dump_index))) }
                </div>
            </body>
        </html>
    );

    write!(output, "{}", html)
}

fn render_class<'a>(class: &'a DumpClass, dump_index: &DumpIndex) -> HtmlContent<'a> {
    let description = class
        .description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    let mut element_class = "dump-class".to_owned();
    if class.tags.contains("Deprecated") {
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
                    { render_type_link(&DumpType::Class(superclass.to_string())) }
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
                    { render_markdown(description, &dump_index) }
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
                            { Fragment::new(class.properties().map(|property| render_property(property, &class.name, dump_index))) }
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
                            { Fragment::new(class.functions().map(|function| render_function(function, &class.name, dump_index))) }
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
                            { Fragment::new(class.events().map(|event| render_event(event, &class.name, dump_index))) }
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
                            { Fragment::new(class.callbacks().map(|callback| render_callback(callback, &class.name, dump_index))) }
                        </div>
                    </div>
                )
            } else {
                HtmlContent::None
            } }
        </div>
    )
}

fn render_property<'a>(
    property: &'a DumpClassProperty,
    parent_name: &str,
    dump_index: &DumpIndex,
) -> HtmlContent<'a> {
    let description = property
        .description
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
                { render_type_link(&property.value_type) }
            </div>
            { render_member_description(description, property.description_source, dump_index) }
        </div>
    )
}

fn render_function<'a>(
    function: &'a DumpClassFunction,
    parent_name: &str,
    dump_index: &DumpIndex,
) -> HtmlContent<'a> {
    let description = function
        .description
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
                ") => "
                { render_return_type(&function.return_type) }
            </div>
            { render_member_description(description, function.description_source, dump_index) }
        </div>
    )
}

fn render_event<'a>(
    event: &'a DumpClassEvent,
    parent_name: &str,
    dump_index: &DumpIndex,
) -> HtmlContent<'a> {
    let description = event
        .description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    let qualified_name = format!("{}.{}", parent_name, event.name);
    let signal_type = DumpType::DataType("RBXScriptSignal".to_string());

    html!(
        <div class={ member_element_class(&event.tags, "dump-class-event") } id={ qualified_name.clone() }>
            <div class="dump-function-signature">
                <a class="dump-class-member-name" href={ format!("#{}", qualified_name)}>
                    { &event.name }
                </a>
                ": "
                { render_type_link(&signal_type) }
                "("
                { render_arguments(&event.parameters) }
                ")"
            </div>
            { render_member_description(description, event.description_source, dump_index) }
        </div>
    )
}

fn render_callback<'a>(
    callback: &'a DumpClassCallback,
    parent_name: &str,
    dump_index: &DumpIndex,
) -> HtmlContent<'a> {
    let description = callback
        .description
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
                ") => "
                { render_return_type(&callback.return_type) }
            </div>
            { render_member_description(description, callback.description_source, dump_index) }
        </div>
    )
}

fn render_member_description<'a>(
    description: &'a str,
    source: Option<ContentSource>,
    dump_index: &DumpIndex,
) -> HtmlContent<'a> {
    html!(
        <div class="dump-class-member-description">
            <div class="dump-class-member-description-text markdown">
                { render_markdown(description, dump_index) }
            </div>
            <div class="dump-class-member-meta">
                { Fragment::new(source.map(|source| html!(
                    <span class="dump-info" title={ format!("Content source: {}", source) } />
                ))) }
            </div>
        </div>
    )
}

fn render_return_type(return_type: &DumpReturnType) -> HtmlContent {
    match return_type {
        DumpReturnType::Single(t) => render_type_link(&t),
        DumpReturnType::Multiple(ts) => html!(
            <span>
            "("
            {
                Fragment::new(ts
                .iter()
                .enumerate()
                .map(|(index, param)| html!(
                    <span class="dump-function-return-type">
                        { render_type_link(&param) }
                        {
                            if index < ts.len() - 1 {
                                ", ".into()
                            } else {
                                HtmlContent::None
                            }
                        }
                    </span>
                )))
            }
            ")"
            </span>
        ),
    }
}

fn render_type_link(t: &DumpType) -> HtmlContent<'static> {
    let name = t.get_name();
    html!(
        <a href={ format!("#{}", name) }>
            { name.to_string() }
        </a>
    )
}

fn render_arguments(parameters: &[DumpFunctionParameter]) -> Fragment {
    Fragment::new(parameters.iter().enumerate().map(|(index, param)| {
        html!(
            <div class="dump-function-argument">
                { &param.name }
                ": "
                { render_type_link(&param.kind) }
                {
                    if index < parameters.len() - 1 {
                        ",".into()
                    } else {
                        HtmlContent::None
                    }
                }
            </div>
        )
    }))
}

fn member_element_class(tags: &BTreeSet<String>, main_class: &str) -> String {
    let mut element_class = "dump-class-member ".to_owned();
    element_class.push_str(main_class);
    if tags.contains("Deprecated") {
        element_class.push_str(" dump-member-deprecated");
    }
    element_class
}

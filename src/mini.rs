use std::{
    fmt::{self, Write},
};

use pulldown_cmark;
use ritz::{html, UnescapedText, HtmlContent, Fragment};

use crate::{
    database::{
        Database,
        Class,
        Property,
        Function,
        FunctionParameter,
        Event,
        Callback,
        Source,
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

pub fn emit_wiki(database: &Database, output: &mut String) -> fmt::Result {
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
                { render_classes(database) }
            </body>
        </html>
    );

    write!(output, "{}", html)
}

fn render_classes(database: &Database) -> HtmlContent {
    let mut keys: Vec<_> = database.classes.keys().collect();
    keys.sort();

    html!(
        <div class="dump-classes">
            { Fragment::new(keys.iter().map(|key| render_class(&database.classes[key.as_str()]))) }
        </div>
    )
}

fn render_class(class: &Class) -> HtmlContent {
    let (description, description_source) = class.description
        .as_ref()
        .map(|entry| (entry.text.as_str(), Some(&entry.source)))
        .unwrap_or((DEFAULT_DESCRIPTION, None));

    html!(
        <div class="dump-class">
            <a class="dump-class-title" id={ &class.name } href={ format!("#{}", class.name) }>
                { &class.name }
            </a>

            { class.superclass.as_ref().map(|superclass| html!(
                <p class="dump-class-inherits">
                    "Inherits: "
                    { render_type_link(&superclass) }
                </p>
            )) }

            /* { if class.tags.len() > 0 {
                html!(
                     <p class="dump-class-tags">
                        "Tags: "
                        { class.tags.join(", ") }
                     </p>
                )
            } else {
                HtmlContent::None
            } } */

            <div class="dump-class-description">
                <div class="dump-class-description-text markdown">
                    { render_markdown(description) }
                </div>
                <div class="dump-class-description-meta">
                    { description_source.map(|source| html!(
                        <span class="dump-info" title={ format!("Content source: {}", source) } />
                    )) }
                </div>
            </div>

            { if !class.properties.is_empty() {
                html!(
                    <div class="dump-class-member-section">
                        <div class="dump-class-subtitle">"Properties"</div>
                        <div class="dump-class-member-section-list">
                            { Fragment::new(class.properties.values().map(|property| render_property(property, &class.name))) }
                        </div>
                    </div>
                )
            } else {
                HtmlContent::None
            } }

            { if !class.functions.is_empty() {
                html!(
                    <div class="dump-class-member-section">
                        <div class="dump-class-subtitle">"Functions"</div>
                        <div class="dump-class-member-section-list">
                            { Fragment::new(class.functions.values().map(|function| render_function(function, &class.name))) }
                        </div>
                    </div>
                )
            } else {
                HtmlContent::None
            } }

            { if !class.events.is_empty() {
                html!(
                    <div class="dump-class-member-section">
                        <div class="dump-class-subtitle">"Events"</div>
                        <div class="dump-class-member-section-list">
                            { Fragment::new(class.events.values().map(|event| render_event(event, &class.name))) }
                        </div>
                    </div>
                )
            } else {
                HtmlContent::None
            } }

            { if !class.callbacks.is_empty() {
                html!(
                    <div class="dump-class-member-section">
                        <div class="dump-class-subtitle">"Callbacks"</div>
                        <div class="dump-class-member-section-list">
                            { Fragment::new(class.callbacks.values().map(|callback| render_callback(callback, &class.name))) }
                        </div>
                    </div>
                )
            } else {
                HtmlContent::None
            } }
        </div>
    )
}

fn render_property<'a>(property: &'a Property, parent_name: &str) -> HtmlContent<'a> {
    let (description, description_source) = property.description
        .as_ref()
        .map(|entry| (entry.text.as_str(), Some(&entry.source)))
        .unwrap_or((DEFAULT_DESCRIPTION, None));

    let qualified_name = format!("{}.{}", parent_name, property.name);

    html!(
        <div class="dump-class-member" id={ qualified_name.clone() }>
            <div class="dump-class-property-signature">
                <a class="dump-class-member-name" href={ format!("#{}", qualified_name) }>
                    { &property.name }
                </a>
                ": "
                /* { render_type_link(&property.value_type.name) } */
            </div>
            { render_member_description(description, description_source) }
        </div>
    )
}

fn render_function<'a>(function: &'a Function, parent_name: &str) -> HtmlContent<'a> {
    // let description = function.description
    //     .as_ref()
    //     .map(String::as_str)
    //     .unwrap_or(DEFAULT_DESCRIPTION);

    // let qualified_name = format!("{}.{}", parent_name, function.name);

    // html!(
    //     <div class="dump-class-member dump-class-function" id={ qualified_name.clone() }>
    //         <div class="dump-function-signature">
    //             <a class="dump-class-member-name" href={ format!("#{}", qualified_name) }>
    //                 { &function.name }
    //             </a>
    //             "("
    //             { render_arguments(&function.parameters) }
    //             "): "
    //             { render_type_link(&function.return_type.name) }
    //         </div>
    //         { render_member_description(description, function.description_source) }
    //     </div>
    // )
    html!(<div>"todo"</div>)
}

fn render_event<'a>(event: &'a Event, parent_name: &str) -> HtmlContent<'a> {
    // let description = event.description
    //     .as_ref()
    //     .map(String::as_str)
    //     .unwrap_or(DEFAULT_DESCRIPTION);

    // let qualified_name = format!("{}.{}", parent_name, event.name);

    // html!(
    //     <div class="dump-class-member dump-class-event" id={ qualified_name.clone() }>
    //         <div class="dump-function-signature">
    //             <a class="dump-class-member-name" href={ format!("#{}", qualified_name)}>
    //                 { &event.name }
    //             </a>
    //             ": "
    //             { render_type_link("RBXScriptSignal") }
    //             "("
    //             { render_arguments(&event.parameters) }
    //             ")"
    //         </div>
    //         { render_member_description(description, event.description_source) }
    //     </div>
    // )
    html!(<div>"todo"</div>)
}

fn render_callback<'a>(callback: &'a Callback, parent_name: &str) -> HtmlContent<'a> {
    // let description = callback.description
    //     .as_ref()
    //     .map(String::as_str)
    //     .unwrap_or(DEFAULT_DESCRIPTION);

    // let qualified_name = format!("{}.{}", parent_name, callback.name);

    // html!(
    //     <div class="dump-class-member dump-class-callback" id={ qualified_name.clone() }>
    //         <div class="dump-function-signature">
    //             <a class="dump-class-member-name" href={ format!("#{}", qualified_name)}>
    //                 { &callback.name }
    //             </a>
    //             ": function("
    //             { render_arguments(&callback.parameters) }
    //             "): "
    //             { render_type_link(&callback.return_type.name) }
    //         </div>
    //         { render_member_description(description, callback.description_source) }
    //     </div>
    // )
    html!(<div>"todo"</div>)
}

fn render_member_description(text: &str, source: Option<&Source>) -> HtmlContent<'static> {
    // html!(
    //     <div class="dump-class-member-description">
    //         <div class="dump-class-member-description-text markdown">
    //             { render_markdown(description) }
    //         </div>
    //         <div class="dump-class-member-meta">
    //             { Fragment::new(source.map(|source| html!(
    //                 <span class="dump-info" title={ format!("Content source: {}", source) } />
    //             ))) }
    //         </div>
    //     </div>
    // )
    html!(<div>"todo"</div>)
}

fn render_type_link(name: &str) -> HtmlContent {
    html!(
        <a href={ format!("#{}", name) }>
            { name }
        </a>
    )
}

fn render_arguments(parameters: &Vec<FunctionParameter>) -> Fragment {
    Fragment::new(parameters
        .iter()
        .enumerate()
        .map(|(index, param)| html!(
            <div class="dump-function-argument">
                { &param.name }
                ": "
                /* { render_type_link(&param.kind.name) } */
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
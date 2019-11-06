use std::{
    collections::BTreeSet,
    fmt::{self, Write},
    fs,
    path::Path,
};

use pulldown_cmark;
use ritz::{html, Fragment, HtmlContent, UnescapedText};

use crate::dump::{
    ContentSource, Dump, DumpClass, DumpClassCallback, DumpClassEvent, DumpClassFunction,
    DumpClassProperty, DumpFunctionParameter, DumpFunctionReturn, DumpIndex, DumpReference,
    DumpType,
};

static STYLE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/resources/miniwiki.css"
));

static DEFAULT_DESCRIPTION: &str = "*No description available.*";

pub trait LinkResolver {
    // returns (url, title)
    fn resolve_reference(&self, reference_str: &str) -> Option<(String, String)>;
    // returns (url, title)
    fn type_link(&self, dump_type: &DumpType) -> Option<(String, String)>;
    
    fn class_id(&self, class_name: &str) -> String;
    fn class_member_id(&self, class_name: &str, member_name: &str) -> String;
}

struct FullSiteLinkResolver {
    dump_index: DumpIndex,
}

impl LinkResolver for FullSiteLinkResolver {
    fn resolve_reference(&self, reference_str: &str) -> Option<(String, String)> {
        match self.dump_index.resolve_reference(reference_str)? {
            DumpReference::Type(dump_type) => self.type_link(&dump_type),
            DumpReference::Member(dump_type, member_name) => match &dump_type {
                DumpType::Class(_name) => {
                    let type_name = dump_type.get_name();
                    let member_title = format!("{}.{}", type_name, member_name);
                    // TODO: move element id generation to LinkResolver
                    let member_link = format!("{}#{}", type_name, member_name);
                    Some((member_link.to_owned(), member_title.to_string()))
                },
                // TODO: support other types
                _ => None
            }
        }
    }
    fn type_link(&self, dump_type: &DumpType) -> Option<(String, String)> {
        match &dump_type {
            DumpType::Class(_name) => {
                let type_name = dump_type.get_name();
                let type_link = format!("{}", type_name);
                Some((type_link, type_name.to_string()))
            }
            // TODO: support other types
            _ => None
        }
    }
    fn class_id(&self, class_name: &str) -> String {
        class_name.to_owned()
    }
    fn class_member_id(&self, _class_name: &str, member_name: &str) -> String {
        member_name.to_owned()
    }
}

struct MiniSiteLinkResolver {
    dump_index: DumpIndex,
}

impl LinkResolver for MiniSiteLinkResolver {
    fn resolve_reference(&self, reference_str: &str) -> Option<(String, String)> {
        match self.dump_index.resolve_reference(reference_str)? {
            DumpReference::Type(dump_type) => self.type_link(&dump_type),
            DumpReference::Member(dump_type, member_name) => {
                let type_name = dump_type.get_name();
                let member_title = format!("{}.{}", type_name, member_name);
                let member_link = format!("#{}.{}", type_name, member_name);
                Some((member_link.to_owned(), member_title.to_string()))
            }
        }
    }
    fn type_link(&self, dump_type: &DumpType) -> Option<(String, String)> {
        let type_name = dump_type.get_name();
        let type_link = format!("#{}", type_name);
        Some((type_link, type_name.to_string()))
    }
    fn class_id(&self, class_name: &str) -> String {
        class_name.to_owned()
    }
    fn class_member_id(&self, class_name: &str, member_name: &str) -> String {
        format!("{}.{}", class_name, member_name)
    }
}

fn render_markdown<'a>(input: &'a str, link_resolver: &dyn LinkResolver) -> HtmlContent<'static> {
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_TABLES);

    let link_callback = |_norm: &str, raw: &str| link_resolver.resolve_reference(&raw.trim());
    let parser =
        pulldown_cmark::Parser::new_with_broken_link_callback(input, options, Some(&link_callback));

    let mut output = String::new();
    pulldown_cmark::html::push_html(&mut output, parser);

    UnescapedText::new(output).into()
}

pub fn emit_full_site(dump: &Dump, path: &Path) {
    let link_resolver = FullSiteLinkResolver {
        dump_index: DumpIndex::new_from_dump(dump),
    };

    let class_folder = path.join("class");
    fs::create_dir_all(&class_folder).expect("Failed to create class folder.");
    for class in dump.classes.iter() {
        let mut output = String::new();
        emit_class_page(&class, &link_resolver, &mut output)
            .expect("Could not generate class page");

        let class_path = class_folder.join(format!("{}.html", &class.name));
        fs::write(class_path, &output).expect("Could not write to output file");
    }
}

fn emit_class_page(
    class: &DumpClass,
    dump_index: &dyn LinkResolver,
    output: &mut String,
) -> fmt::Result {
    writeln!(output, "<!doctype html>")?;

    let html = html!(
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width,initial-scale=1,shrink-to-fit=no" />
                <style>{ UnescapedText::new(STYLE) }</style>
                <title>{ &class.name }</title>
            </head>
            <body>
                <div class="dump-classes">
                    { render_class(class, dump_index) }
                </div>
            </body>
        </html>
    );

    write!(output, "{}", html)
}

pub fn emit_wiki(dump: &Dump, output: &mut String) -> fmt::Result {
    writeln!(output, "<!doctype html>")?;

    let link_resolver = MiniSiteLinkResolver {
        dump_index: DumpIndex::new_from_dump(&dump),
    };

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
                    { Fragment::new(dump.classes.iter().map(|class| render_class(class, &link_resolver))) }
                </div>
            </body>
        </html>
    );

    write!(output, "{}", html)
}

fn render_class<'a>(class: &'a DumpClass, link_resolver: &dyn LinkResolver) -> HtmlContent<'a> {
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
                    { render_type_link(&DumpType::Class(superclass.to_string()), link_resolver) }
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
                    { render_markdown(description, link_resolver) }
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
                            { Fragment::new(class.properties().map(|property| render_property(property, &class.name, link_resolver))) }
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
                            { Fragment::new(class.functions().map(|function| render_function(function, &class.name, link_resolver))) }
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
                            { Fragment::new(class.events().map(|event| render_event(event, &class.name, link_resolver))) }
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
                            { Fragment::new(class.callbacks().map(|callback| render_callback(callback, &class.name, link_resolver))) }
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
    link_resolver: &dyn LinkResolver,
) -> HtmlContent<'a> {
    let description = property
        .description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    let id = link_resolver.class_member_id(parent_name, &property.name);
    html!(
        <div class={ member_element_class(&property.tags, "dump-class-property") } id={ id.clone() }>
            <div class="dump-class-property-signature">
                <a class="dump-class-member-name" href={ format!("#{}", id) }>
                    { &property.name }
                </a>
                ": "
                { render_type_link(&property.value_type, link_resolver) }
            </div>
            { render_member_description(description, property.description_source, link_resolver) }
        </div>
    )
}

fn render_function<'a>(
    function: &'a DumpClassFunction,
    parent_name: &str,
    link_resolver: &dyn LinkResolver,
) -> HtmlContent<'a> {
    let description = function
        .description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    let id = link_resolver.class_member_id(parent_name, &function.name);
    html!(
        <div class={ member_element_class(&function.tags, "dump-class-function") } id={ id.clone() }>
            <div class="dump-function-signature">
                <a class="dump-class-member-name" href={ format!("#{}", id) }>
                    { &function.name }
                </a>
                "("
                { render_arguments(&function.parameters, link_resolver) }
                ") => "
                { render_return_type(&function.return_type, &function.returns, link_resolver) }
            </div>
            { render_member_description(description, function.description_source, link_resolver) }
            {
                if function.parameters.iter().any(|x| x.description.is_some()) {
                    render_parameters_table(&function.parameters, link_resolver)
                } else {
                    HtmlContent::None
                }
            }
            {
                if !function.returns.is_empty() {
                    render_returns_table(&function.returns, link_resolver)
                } else {
                    HtmlContent::None
                }
            }
        </div>
    )
}

fn render_event<'a>(
    event: &'a DumpClassEvent,
    parent_name: &str,
    link_resolver: &dyn LinkResolver,
) -> HtmlContent<'a> {
    let description = event
        .description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    let signal_type = DumpType::DataType("RBXScriptSignal".to_string());

    let id = link_resolver.class_member_id(parent_name, &event.name);
    html!(
        <div class={ member_element_class(&event.tags, "dump-class-event") } id={ id.clone() }>
            <div class="dump-function-signature">
                <a class="dump-class-member-name" href={ format!("#{}", id)}>
                    { &event.name }
                </a>
                ": "
                { render_type_link(&signal_type, link_resolver) }
                "("
                { render_arguments(&event.parameters, link_resolver) }
                ")"
            </div>
            { render_member_description(description, event.description_source, link_resolver) }
        </div>
    )
}

fn render_callback<'a>(
    callback: &'a DumpClassCallback,
    parent_name: &str,
    link_resolver: &dyn LinkResolver,
) -> HtmlContent<'a> {
    let description = callback
        .description
        .as_ref()
        .map(String::as_str)
        .unwrap_or(DEFAULT_DESCRIPTION);

    let id = link_resolver.class_member_id(parent_name, &callback.name);
    html!(
        <div class={ member_element_class(&callback.tags, "dump-class-callback") } id={ id.clone() }>
            <div class="dump-function-signature">
                <a class="dump-class-member-name" href={ format!("#{}", id)}>
                    { &callback.name }
                </a>
                ": function("
                { render_arguments(&callback.parameters, link_resolver) }
                ") => "
                { render_return_type(&callback.return_type, &callback.returns, link_resolver) }
            </div>
            { render_member_description(description, callback.description_source, link_resolver) }
        </div>
    )
}

fn render_member_description<'a>(
    description: &'a str,
    source: Option<ContentSource>,
    link_callback: &dyn LinkResolver,
) -> HtmlContent<'a> {
    html!(
        <div class="dump-class-member-description">
            <div class="dump-class-member-description-text markdown">
                { render_markdown(description, link_callback) }
            </div>
            <div class="dump-class-member-meta">
                { Fragment::new(source.map(|source| html!(
                    <span class="dump-info" title={ format!("Content source: {}", source) } />
                ))) }
            </div>
        </div>
    )
}

fn render_return_type(
    return_type: &DumpType,
    returns: &[DumpFunctionReturn],
    link_resolver: &dyn LinkResolver,
) -> HtmlContent<'static> {
    if returns.is_empty() {
        render_type_link(&return_type, link_resolver)
    } else if returns.len() == 1 {
        render_type_link(&returns[0].kind, link_resolver)
    } else {
        html!(
            <span>
            "("
            {
                Fragment::new(returns
                .iter()
                .enumerate()
                .map(|(index, param)| html!(
                    <span class="dump-function-return-type">
                        { render_type_link(&param.kind, link_resolver) }
                        {
                            if index < returns.len() - 1 {
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
        )
    }
}

fn render_type_link(
    dump_type: &DumpType,
    link_resolver: &dyn LinkResolver,
) -> HtmlContent<'static> {
    match link_resolver.type_link(dump_type) {
        Some((url, title)) => html!(
            <a href={ url } title={ title }>
                { dump_type.get_name().to_owned() }
            </a>
        ),
        None => html!(
            <span>
                { dump_type.get_name().to_owned() }
            </span>
        ),
    }
}

fn render_arguments<'a>(
    parameters: &'a [DumpFunctionParameter],
    link_resolver: &dyn LinkResolver,
) -> Fragment<'a> {
    Fragment::new(parameters.iter().enumerate().map(|(index, param)| {
        html!(
            <div class="dump-function-argument">
                { &param.name }
                ": "
                { render_type_link(&param.kind, link_resolver) }
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

fn render_parameters_table<'a>(
    parameters: &'a [DumpFunctionParameter],
    link_resolver: &dyn LinkResolver,
) -> HtmlContent<'a> {
    let has_defaults = parameters.iter().any(|p| p.default.is_some());
    html!(
        <div class="dump-function-returns">
            <div class="dump-class-member-subtitle">"Parameters"</div>
            <table>
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Type"</th>
                        {
                            if has_defaults {
                                html!(<th>"Default"</th>)
                            } else {
                                HtmlContent::None
                            }
                        }
                        <th>"Description"</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        Fragment::new(parameters
                            .iter()
                            .enumerate()
                            .map(|(index, val)| html!(
                                <tr>
                                <td>{ &val.name }</td>
                                <td>{ render_type_link(&val.kind, link_resolver) }</td>
                                {
                                    if has_defaults {
                                        html!(
                                            <td>
                                            {
                                                if let Some(default) = &val.default {
                                                    html!({ default })
                                                } else {
                                                    HtmlContent::None
                                                }
                                            }
                                            </td>
                                        )
                                    } else {
                                        HtmlContent::None
                                    }
                                }
                                <td>
                                {
                                    if let Some(description) = &val.description {
                                        render_markdown(&description, link_resolver)
                                    } else {
                                        HtmlContent::None
                                    }
                                }
                                </td>
                                </tr>
                            )))
                    }
                </tbody>
            </table>
        </div>
    )
}

fn render_returns_table<'a>(
    returns: &'a [DumpFunctionReturn],
    link_resolver: &dyn LinkResolver,
) -> HtmlContent<'a> {
    html!(
        <div class="dump-function-returns">
            <div class="dump-class-member-subtitle">"Returns"</div>
            <table>
                <thead>
                    <tr>
                        <th>"Type"</th>
                        <th>"Description"</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        Fragment::new(returns
                            .iter()
                            .enumerate()
                            .map(|(index, val)| html!(
                                <tr>
                                <td>{ render_type_link(&val.kind, link_resolver) }</td>
                                <td>
                                {
                                    if let Some(description) = &val.description {
                                        render_markdown(&description, link_resolver)
                                    } else {
                                        HtmlContent::None
                                    }
                                }
                                </td>
                                </tr>
                            )))
                    }
                </tbody>
            </table>
        </div>
    )
}

fn member_element_class(tags: &BTreeSet<String>, main_class: &str) -> String {
    let mut element_class = "dump-class-member ".to_owned();
    element_class.push_str(main_class);
    if tags.contains("Deprecated") {
        element_class.push_str(" dump-member-deprecated");
    }
    element_class
}

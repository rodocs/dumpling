//! This is a set of pretty horrible hacks and experiments to invent a
//! templating language for HTML in Rust.

use std::{
    collections::HashMap,
    fmt::{self, Write},
};

/// Utility to HTML escape a string when using format strings.
/// Inspired by https://github.com/rust-lang/rust/blob/master/src/librustdoc/html/escape.rs
/// This functionality should totally become a crate.
pub struct HtmlEscape<'a>(pub &'a str);

impl<'a> fmt::Display for HtmlEscape<'a> {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        // This could be more efficient -- copies will happen one character at a
        // time regardless of escapes.
        for value in self.0.chars() {
            match value {
                '"' => output.write_str("&quot;")?,
                '&' => output.write_str("&amp;")?,
                '\'' => output.write_str("&#x27;")?,
                '<' => output.write_str("&lt;")?,
                '>' => output.write_str("&gt;")?,
                _ => output.write_char(value)?,
            }
        }

        Ok(())
    }
}

pub fn tag(name: &str) -> HtmlTag {
    HtmlTag {
        name: name.to_string(),
        content: Vec::new(),
        attributes: HashMap::new(),
    }
}

pub fn tag_class(name: &str, class: &str) -> HtmlTag {
    let mut attributes = HashMap::new();
    attributes.insert("class".to_string(), class.to_string());

    HtmlTag {
        name: name.to_string(),
        content: Vec::new(),
        attributes,
    }
}

pub enum MaybeHtmlContent {
    Some(HtmlContent),
    None,
}

impl<T> From<T> for MaybeHtmlContent where T: Into<HtmlContent> {
    fn from(value: T) -> MaybeHtmlContent {
        MaybeHtmlContent::Some(value.into())
    }
}

impl<T> From<Option<T>> for MaybeHtmlContent where T: Into<HtmlContent> {
    fn from(value: Option<T>) -> MaybeHtmlContent {
        match value {
            Some(content) => MaybeHtmlContent::Some(content.into()),
            None => MaybeHtmlContent::None,
        }
    }
}

pub struct HtmlTag {
    name: String,
    content: Vec<HtmlContent>,
    attributes: HashMap<String, String>,
}

impl HtmlTag {
    pub fn attr(mut self, key: &str, value: &str) -> HtmlTag {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }

    pub fn class(mut self, class: &str) -> HtmlTag {
        self.attributes.insert("class".to_string(), class.to_string());
        self
    }

    pub fn children<T, I>(mut self, iterator: I) -> HtmlTag
        where T: Into<MaybeHtmlContent>, I: IntoIterator<Item = T>
    {
        for child in iterator {
            match child.into() {
                MaybeHtmlContent::Some(content) => self.content.push(content),
                MaybeHtmlContent::None => {},
            }
        }
        self
    }

    pub fn child<T>(mut self, child: T) -> HtmlTag
        where T: Into<MaybeHtmlContent>
    {
        match child.into() {
            MaybeHtmlContent::Some(content) => self.content.push(content),
            MaybeHtmlContent::None => {},
        }
        self
    }

    pub fn add_child<T>(&mut self, child: T)
        where T: Into<MaybeHtmlContent>
    {
        match child.into() {
            MaybeHtmlContent::Some(content) => self.content.push(content),
            MaybeHtmlContent::None => {},
        }
    }

    pub fn add_children<T, I>(&mut self, iterator: I)
        where T: Into<MaybeHtmlContent>, I: IntoIterator<Item = T>
    {
        for child in iterator {
            match child.into() {
                MaybeHtmlContent::Some(content) => self.content.push(content),
                MaybeHtmlContent::None => {},
            }
        }
    }

    pub fn child_count(&self) -> usize {
        self.content.len()
    }
}

impl fmt::Display for HtmlTag {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        write!(output, "<{}", self.name)?;

        for (key, value) in &self.attributes {
            write!(output, " {}=\"{}\"", key, value)?;
        }

        write!(output, ">")?;

        for content in &self.content {
            write!(output, "{}", content)?;
        }

        write!(output, "</{}>", self.name)?;

        Ok(())
    }
}

pub enum HtmlContent {
    Text(String),
    Tag(HtmlTag),
}

impl fmt::Display for HtmlContent {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HtmlContent::Text(text) => write!(output, "{}", HtmlEscape(text))?,
            HtmlContent::Tag(tag) => write!(output, "{}", tag)?,
        }

        Ok(())
    }
}

impl<'a> From<&'a str> for HtmlContent {
    fn from(value: &'a str) -> HtmlContent {
        HtmlContent::Text(value.to_string())
    }
}

impl<'a> From<&'a String> for HtmlContent {
    fn from(value: &'a String) -> HtmlContent {
        HtmlContent::Text(value.clone())
    }
}

impl From<HtmlTag> for HtmlContent {
    fn from(value: HtmlTag) -> HtmlContent {
        HtmlContent::Tag(value)
    }
}
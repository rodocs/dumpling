use std::{
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
        class: None,
        content: Vec::new(),
    }
}

pub struct HtmlTag {
    name: String,
    class: Option<String>,
    content: Vec<HtmlContent>,
}

impl HtmlTag {
    pub fn with_class(mut self, class: &str) -> HtmlTag {
        self.class = Some(class.to_string());
        self
    }

    pub fn add_children<T, I>(&mut self, iterator: I) -> &mut HtmlTag
        where T: Into<HtmlContent>, I: IntoIterator<Item = T>
    {
        for child in iterator {
            self.content.push(child.into());
        }
        self
    }

    pub fn append<T>(mut self, child: T) -> HtmlTag
        where T: Into<HtmlContent>
    {
        self.content.push(child.into());
        self
    }
}

impl fmt::Display for HtmlTag {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        write!(output, "<{}", self.name)?;

        if let Some(class) = &self.class {
            write!(output, " class=\"{}\"", class)?;
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

impl From<HtmlTag> for HtmlContent {
    fn from(value: HtmlTag) -> HtmlContent {
        HtmlContent::Tag(value)
    }
}
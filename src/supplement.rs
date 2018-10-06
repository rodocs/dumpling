//! Used to parse and represent supplemental/prose data that should be attached
//! to the API dump material.
//!
//! These files are a series of item descriptions, each of which is a section of
//! fenced TOML followed by arbitrary Markdown prose.
//!
//! A document describing a portion of Instance might look like:
//!
//! ```md
//! +++
//! target = "Instance"
//! +++
//!
//! The base class for all Roblox instances.
//!
//! +++
//! target = "Instance.Name"
//! +++
//!
//! A handy name to refer to the `Instance` with.
//! ```

use std::collections::HashMap;

use toml;
use serde_derive::{Serialize, Deserialize};

// This is inspired by Hugo's TOML front-matter indicator
// --- is used for YAML, and +++ is used to disambiguate for TOML.
const METADATA_FENCE: &str = "+++";

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemDescription {
    metadata: Metadata,
    prose: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    target: String,
}

#[derive(Debug)]
pub enum ParseError {
    UnclosedMetadataBlock,
    InvalidToml(toml::de::Error),
}

impl From<toml::de::Error> for ParseError {
    fn from(error: toml::de::Error) -> ParseError {
        ParseError::InvalidToml(error)
    }
}

pub fn parse(source: &str) -> Result<HashMap<String, ItemDescription>, ParseError> {
    let mut result = HashMap::new();

    let mut fence_locations = source.match_indices(METADATA_FENCE).peekable();

    loop {
        match fence_locations.next() {
            Some((start_index, fence)) => {
                let (end_index, _) = fence_locations.next()
                    .ok_or(ParseError::UnclosedMetadataBlock)?;

                let metadata_source = &source[(start_index + fence.len())..end_index].trim();
                let metadata: Metadata = toml::from_str(metadata_source)?;

                let prose_after_end_index = match fence_locations.peek() {
                    Some((index, _)) => *index,
                    None => source.len(),
                };

                let prose = source[(end_index + fence.len())..prose_after_end_index].trim().to_string();

                result.insert(metadata.target.clone(), ItemDescription {
                    metadata,
                    prose,
                });
            },
            None => break,
        }
    }

    Ok(result)
}
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

use std::{
    collections::HashMap,
    fs,
    io,
    path::Path,
};

use toml;

// This is inspired by Hugo's TOML front-matter indicator
// --- is used for YAML, and +++ is used to disambiguate for TOML.
const METADATA_FENCE: &str = "+++";

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemDescription {
    pub metadata: Metadata,
    pub prose: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub target: String,
}

#[derive(Debug)]
pub enum ReadError {
    IoError(io::Error),
    ParseError(ParseError),
}

impl From<io::Error> for ReadError {
    fn from(error: io::Error) -> ReadError {
        ReadError::IoError(error)
    }
}

impl From<ParseError> for ReadError {
    fn from(error: ParseError) -> ReadError {
        ReadError::ParseError(error)
    }
}

fn read_all_internal(path: &Path, output: &mut HashMap<String, ItemDescription>) -> Result<(), ReadError> {
    let metadata = fs::metadata(path)?;

    if metadata.is_file() {
        let contents = fs::read_to_string(path)?;
        parse(&contents, output)?;

        Ok(())
    } else if metadata.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            read_all_internal(&entry_path, output)?;
        }

        Ok(())
    } else {
        unimplemented!();
    }
}

/// Recurisively loads all supplemental Markdown files from an input file or
/// folder.
pub fn read_all(path: &Path) -> Result<HashMap<String, ItemDescription>, ReadError> {
    let mut result = HashMap::new();

    read_all_internal(path, &mut result)?;

    Ok(result)
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

pub fn parse(source: &str, output: &mut HashMap<String, ItemDescription>) -> Result<(), ParseError> {
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

                output.insert(metadata.target.clone(), ItemDescription {
                    metadata,
                    prose,
                });
            },
            None => break,
        }
    }

    Ok(())
}
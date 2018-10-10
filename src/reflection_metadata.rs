//! Parses ReflectionMetadata.xml to pull out descriptions and other
//! information.

use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead},
    path::Path,
};

use quick_xml::{
    Reader,
    events::{
        BytesStart,
        Event,
        attributes::Attributes,
    },
};

fn extract_attributes<B: BufRead>(reader: &Reader<B>, attributes: Attributes) -> HashMap<String, String> {
    let mut output = HashMap::new();

    for attribute in attributes {
        let attribute = attribute.unwrap();
        let key = reader.decode(&attribute.key).to_string();
        let value = reader.decode(&attribute.value).to_string();

        output.insert(key, value);
    }

    output
}

type XmlQuery = (&'static str, Option<Vec<(&'static str, &'static str)>>);

fn element_stack_matches<B: BufRead>(reader: &Reader<B>, element_stack: &[BytesStart<'static>], query: &[XmlQuery]) -> bool {
    if element_stack.len() != query.len() {
        return false;
    }

    for (index, element) in element_stack.iter().enumerate() {
        let (expected_tag_name, expected_attributes) = &query[index];

        let tag_name = reader.decode(element.name());

        if tag_name != *expected_tag_name {
            return false;
        }

        if let Some(expected_attributes) = expected_attributes {
            let mut element_attributes = extract_attributes(reader, element.attributes());

            for (key, expected_value) in expected_attributes {
                match element_attributes.get(*key) {
                    Some(value) => {
                        if value != expected_value {
                            return false;
                        }
                    },
                    None => return false,
                }
            }
        }
    }

    return true;
}

#[derive(Debug)]
pub struct ReflectionMetadata {
    pub classes: HashMap<String, ReflectionMetadataClass>,
}

impl ReflectionMetadata {
    pub fn read_from_file(path: &Path) -> io::Result<ReflectionMetadata> {
        let contents = fs::read_to_string(path)?;

        let mut classes = HashMap::new();

        let mut reader = Reader::from_str(&contents);
        reader.trim_text(true);

        let mut xml_buffer = Vec::new();
        let mut element_stack: Vec<BytesStart<'static>> = Vec::new();

        lazy_static! {
            static ref CLASS_QUERY: Vec<XmlQuery> = vec![
                ("roblox", None),
                ("Item", None),
                ("Item", Some(vec![("class", "ReflectionMetadataClass")])),
            ];
        }

        loop {
            match reader.read_event(&mut xml_buffer) {
                Ok(Event::Start(element)) => {
                    element_stack.push(element.into_owned());

                    if element_stack_matches(&reader, &element_stack, &CLASS_QUERY) {
                        let class = ReflectionMetadataClass::decode(&mut reader);
                        classes.insert(class.name.clone(), class);
                    }
                },
                Ok(Event::End(_)) => {
                    element_stack.pop();
                },
                Ok(Event::Eof) => break,
                Err(_) => panic!("Error parsing XML!"),
                _ => {},
            }

            xml_buffer.clear();
        }

        Ok(ReflectionMetadata {
            classes,
        })
    }
}

#[derive(Debug)]
pub struct ReflectionMetadataClass {
    pub name: String,
    pub summary: String,
}

impl ReflectionMetadataClass {
    fn decode<B: BufRead>(reader: &mut Reader<B>) -> ReflectionMetadataClass {
        let mut name = String::new();
        let mut summary = String::new();

        let mut xml_buffer = Vec::new();
        let mut element_stack: Vec<BytesStart<'static>> = Vec::new();

        lazy_static! {
            static ref NAME_QUERY: Vec<XmlQuery> = vec![
                ("Properties", None),
                ("string", Some(vec![("name", "Name")])),
            ];

            static ref SUMMARY_QUERY: Vec<XmlQuery> = vec![
                ("Properties", None),
                ("string", Some(vec![("name", "summary")])),
            ];
        }

        loop {
            match reader.read_event(&mut xml_buffer) {
                Ok(Event::Start(element)) => {
                    element_stack.push(element.into_owned());
                },
                Ok(Event::End(_)) => {
                    if element_stack.len() == 0 {
                        break;
                    }

                    element_stack.pop();
                },
                Ok(Event::Text(text)) => {
                    if element_stack_matches(&reader, &element_stack, &NAME_QUERY) {
                        name = text.unescape_and_decode(reader).unwrap();
                    } else if element_stack_matches(&reader, &element_stack, &SUMMARY_QUERY) {
                        summary = text.unescape_and_decode(reader).unwrap();
                    }
                },
                Ok(Event::Eof) => break,
                Err(_) => panic!("Error parsing ReflectionMetadataClass!"),
                _ => {},
            }

            xml_buffer.clear();
        }

        ReflectionMetadataClass {
            name,
            summary,
        }
    }
}
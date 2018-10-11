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

struct XmlQuery {
    pieces: Vec<(&'static str, Vec<(&'static str, &'static str)>)>,
}

impl XmlQuery {
    pub fn new(query: &[(&'static str, &[(&'static str, &'static str)])]) -> XmlQuery {
        let pieces = query
            .iter()
            .map(|(tag_name, input_attributes)| (*tag_name, input_attributes.iter().cloned().collect::<Vec<_>>()))
            .collect::<Vec<_>>();

        XmlQuery {
            pieces,
        }
    }

    pub fn matches<B: BufRead>(&self, reader: &Reader<B>, element_stack: &[BytesStart<'static>]) -> bool {
        if element_stack.len() != self.pieces.len() {
            return false;
        }

        for (index, element) in element_stack.iter().enumerate() {
            let (expected_tag_name, expected_attributes) = &self.pieces[index];

            let tag_name = reader.decode(element.name());

            if tag_name != *expected_tag_name {
                return false;
            }

            if expected_attributes.len() > 0 {
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
}

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
            static ref CLASS_QUERY: XmlQuery = XmlQuery::new(&[
                ("roblox", &[]),
                ("Item", &[]),
                ("Item", &[("class", "ReflectionMetadataClass")]),
            ]);
        }

        loop {
            match reader.read_event(&mut xml_buffer) {
                Ok(Event::Start(element)) => {
                    element_stack.push(element.into_owned());

                    if CLASS_QUERY.matches(&reader, &element_stack) {
                        let class = ReflectionMetadataClass::decode(&mut reader, &mut element_stack);
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
    pub members: HashMap<String, ReflectionMetadataMember>,
}

impl ReflectionMetadataClass {
    fn decode<B: BufRead>(reader: &mut Reader<B>, element_stack: &mut Vec<BytesStart<'static>>) -> ReflectionMetadataClass {
        let mut name = String::new();
        let mut summary = String::new();
        let mut members = HashMap::new();

        let start_stack_len = element_stack.len();
        let mut xml_buffer = Vec::new();

        lazy_static! {
            static ref NAME_QUERY: XmlQuery = XmlQuery::new(&[
                ("Properties", &[]),
                ("string", &[("name", "Name")]),
            ]);

            static ref SUMMARY_QUERY: XmlQuery = XmlQuery::new(&[
                ("Properties", &[]),
                ("string", &[("name", "summary")]),
            ]);

            static ref MEMBER_QUERY: XmlQuery = XmlQuery::new(&[
                ("Item", &[]), // class is "ReflectionMetadataFunctions" or similar; we don't care.
                ("Item", &[("class", "ReflectionMetadataMember")]),
            ]);
        }

        loop {
            match reader.read_event(&mut xml_buffer) {
                Ok(Event::Start(element)) => {
                    element_stack.push(element.into_owned());

                    if MEMBER_QUERY.matches(&reader, &element_stack[start_stack_len..]) {
                        let member = ReflectionMetadataMember::decode(reader, element_stack);

                        members.insert(member.name.clone(), member);
                    }
                },
                Ok(Event::End(_)) => {
                    element_stack.pop();

                    if element_stack.len() < start_stack_len {
                        break;
                    }
                },
                Ok(Event::Text(text)) => {
                    let relevant_stack = &element_stack[start_stack_len..];

                    if NAME_QUERY.matches(&reader, relevant_stack) {
                        name = text.unescape_and_decode(reader).unwrap();
                    } else if SUMMARY_QUERY.matches(&reader, relevant_stack) {
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
            members,
        }
    }
}

#[derive(Debug)]
pub struct ReflectionMetadataMember {
    pub name: String,
    pub summary: String,
}

impl ReflectionMetadataMember {
    fn decode<B: BufRead>(reader: &mut Reader<B>, element_stack: &mut Vec<BytesStart<'static>>) -> ReflectionMetadataMember {
        let mut name = String::new();
        let mut summary = String::new();

        let start_stack_len = element_stack.len();
        let mut xml_buffer = Vec::new();

        lazy_static! {
            static ref NAME_QUERY: XmlQuery = XmlQuery::new(&[
                ("Properties", &[]),
                ("string", &[("name", "Name")]),
            ]);

            static ref SUMMARY_QUERY: XmlQuery = XmlQuery::new(&[
                ("Properties", &[]),
                ("string", &[("name", "summary")]),
            ]);
        }

        loop {
            match reader.read_event(&mut xml_buffer) {
                Ok(Event::Start(element)) => {
                    element_stack.push(element.into_owned());
                },
                Ok(Event::End(_)) => {
                    element_stack.pop();

                    if element_stack.len() < start_stack_len {
                        break;
                    }
                },
                Ok(Event::Text(text)) => {
                    let relevant_stack = &element_stack[start_stack_len..];

                    if NAME_QUERY.matches(&reader, relevant_stack) {
                        name = text.unescape_and_decode(reader).unwrap();
                    } else if SUMMARY_QUERY.matches(&reader, relevant_stack) {
                        summary = text.unescape_and_decode(reader).unwrap();
                    }
                },
                Ok(Event::Eof) => break,
                Err(_) => panic!("Error parsing ReflectionMetadataClass!"),
                _ => {},
            }

            xml_buffer.clear();
        }

        ReflectionMetadataMember {
            name,
            summary,
        }
    }
}
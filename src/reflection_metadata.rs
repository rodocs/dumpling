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
        Event,
        attributes::Attributes,
    },
};

fn get_attribute<B: BufRead>(key: &str, reader: &Reader<B>, attributes: Attributes) -> Option<String> {
    for attribute in attributes {
        let attribute = attribute.unwrap();
        let this_key = reader.decode(attribute.key);

        if this_key == key {
            let value = reader.decode(&attribute.value);

            return Some(value.to_string());
        }
    }

    None
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

        loop {
            match reader.read_event(&mut xml_buffer) {
                Ok(Event::Start(element)) => {
                    let element_name = reader.decode(element.name());

                    if element_name == "Item" {
                        if let Some(class_name) = get_attribute("class", &reader, element.attributes()) {
                            if class_name == "ReflectionMetadataClass" {
                                let class = ReflectionMetadataClass::decode(&mut reader);
                                classes.insert(class.name.clone(), class);
                            }
                        }
                    }
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

        let mut depth: u32 = 1;
        let mut xml_buffer = Vec::new();
        let mut in_properties = false;
        let mut in_name = false;
        let mut in_summary = false;

        loop {
            match reader.read_event(&mut xml_buffer) {
                Ok(Event::Start(element)) => {
                    depth += 1;

                    let element_name = reader.decode(element.name());

                    if element_name == "Properties" && depth == 2 {
                        in_properties = true;
                        continue;
                    }

                    if in_properties {
                        match get_attribute("name", reader, element.attributes()) {
                            Some(property_name) => {
                                match property_name.as_str() {
                                    "Name" => { in_name = true },
                                    "summary" => { in_summary = true },
                                    _ => {},
                                }
                            },
                            None => {},
                        }
                    }
                },
                Ok(Event::End(element)) => {
                    depth -= 1;

                    if depth == 0 {
                        break;
                    }

                    in_name = false;
                    in_summary = false;

                    let element_name = reader.decode(element.name());

                    if element_name == "Properties" && depth == 1 {
                        in_properties = false;
                        continue;
                    }
                },
                Ok(Event::Text(text)) => {
                    if in_properties {
                        if in_name {
                            name = text.unescape_and_decode(reader).unwrap();
                        } else if in_summary {
                            summary = text.unescape_and_decode(reader).unwrap();
                        }
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
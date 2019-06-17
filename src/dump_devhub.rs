use std::collections::HashMap;

use crate::{
    devhub,
    dump::Dump,
};

pub struct DevHubData {
    pub classes: HashMap<String, DevHubClass>,
}

impl DevHubData {
    pub fn fetch(dump: &Dump) -> DevHubData {
        let mut classes = HashMap::new();

        for class in &dump.classes {
            if let Some(page) = request_instance_page(&class.name) {
                classes.insert(class.name.clone(), instance_page_to_class(&page));
            }
        }

        DevHubData {
            classes,
        }
    }
}

pub struct DevHubClass {
    pub name: String,
    pub description: String,
    pub properties: Vec<DevHubProperty>,
}

pub struct DevHubProperty {
    pub name: String,
    pub description: String,
}

fn request_instance_page(name: &str) -> Option<devhub::InstancePage> {
    let url = format!("https://developer.roblox.com/api-reference/class/{}.json", name);

    println!("Requesting {}", url);

    let mut response = reqwest::get(&url).ok()?;

    if !response.status().is_success() {
        println!("Request failed.");
        return None;
    }

    response.json().expect("Couldn't parse JSON response")
}

fn instance_page_to_class(page: &devhub::InstancePage) -> DevHubClass {
    let class = &page.entry.modular_blocks[0].api_class_section.current_class[0];

    let mut properties = Vec::new();

    for property in &class.property {
        properties.push(DevHubProperty {
            name: property.display_title.clone(),
            description: property.description.as_ref().map(|v| v.clone()).unwrap_or_else(String::new),
        });
    }

    DevHubClass {
        name: class.title.clone(),
        description: class.description.as_ref().map(|v| v.clone()).unwrap_or_else(String::new),
        properties,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_part_page() {
        // FIXME: This seems like a pretty poor unit test, hitting the network!
        let _part = request_instance_page("Part");
    }
}
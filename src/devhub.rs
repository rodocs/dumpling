use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InstancePage {
    pub entry: Entry,
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub modular_blocks: Vec<ModularBlock>,
}

#[derive(Debug, Deserialize)]
pub struct ModularBlock {
    pub api_class_section: ClassSection,
}

#[derive(Debug, Deserialize)]
pub struct ClassSection {
    pub current_class: Vec<Class>,
    pub inherited_class: Vec<Class>,
}

#[derive(Debug, Deserialize)]
pub struct Class {
    pub title: String,
    pub description: Option<String>,
    pub property: Vec<Property>,
}

#[derive(Debug, Deserialize)]
pub struct Property {
    pub title: String,
    pub display_title: String,
    pub description: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = include_str!("../test-files/devhub-Part.json");

    #[test]
    fn deserialize() {
        let _data: InstancePage = serde_json::from_str(EXAMPLE).unwrap();
    }
}

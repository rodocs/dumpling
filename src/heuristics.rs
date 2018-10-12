//! Contains heuristics to guess at and clean up content sourced from outside
//! Dumping itself.

use ::{
    dump::{ContentSource, Dump},
};

/// A handy function to capitalize a string, based on a good solution from:
/// https://stackoverflow.com/a/38406885/802794
fn make_first_letter_uppercase(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Members with camelCase names are probably deprecated.
pub fn camelcase_members_probably_deprecated(dump: &mut Dump) {
    for class in dump.classes.iter_mut() {
        let mut fixups: Vec<(usize, String)> = Vec::new();

        for (index, member) in class.members.iter().enumerate() {
            let first_char = member.get_name().chars().nth(0).unwrap();

            if first_char.is_lowercase() {
                let fixed_name = make_first_letter_uppercase(member.get_name());

                // We should make sure a PascalCase version exists!
                let has_pascal_version = class.members
                    .iter()
                    .position(|member| member.get_name() == fixed_name)
                    .is_some();

                if has_pascal_version {
                    fixups.push((index, fixed_name));
                }
            }
        }

        for (index, fixed_name) in &fixups {
            let member = &mut class.members[*index];

            let description = format!("`{}` is deprecated. Use `{}` instead.", member.get_name(), fixed_name);
            member.set_description(description, ContentSource::Heuristic);
        }
    }
}
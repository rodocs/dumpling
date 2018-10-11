//! Contains heuristics to guess at and clean up content sourced from outside
//! Dumping itself.

use dump::Dump;

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
        for member in class.members.iter_mut() {
            let first_char = member.get_name().chars().nth(0).unwrap();

            if first_char.is_lowercase() {
                let description = {
                    let name = member.get_name();
                    let fixed_name = make_first_letter_uppercase(name);

                    format!("`{}` is camelCase and is probably deprecated. Consider using `{}` instead if it exists.", name, fixed_name)
                };
                member.set_description(description);
            }
        }
    }
}
#[macro_use] extern crate serde_derive;

extern crate clap;
extern crate pulldown_cmark;
extern crate quick_xml;
extern crate serde;
extern crate serde_json;
extern crate toml;

pub mod dump;
pub mod miniwiki;
pub mod supplement;
pub mod templating;
pub mod reflection_metadata;

use std::path::Path;

use clap::{
    App,
    SubCommand,
    Arg,
};

use ::{
    dump::Dump,
};

fn load_dump(dump_path: &Path, supplemental_path: &Path) -> Dump {
    let mut dump = Dump::read_from_file(dump_path)
        .expect("Could not load JSON API dump");

    let supplemental = supplement::read_all(supplemental_path)
        .expect("Could not load supplemental data");

    for class in dump.classes.iter_mut() {
        match supplemental.get(&class.name) {
            Some(description) => {
                class.description = Some(description.prose.clone());
            },
            None => {},
        }
    }

    dump
}

fn miniwiki(dump_path: &Path, supplemental_path: &Path) {
    let dump = load_dump(dump_path, supplemental_path);

    let mut output = String::new();
    miniwiki::emit_wiki(&dump, &mut output).unwrap();

    println!("{}", output);
}

fn megadump(dump_path: &Path, supplemental_path: &Path) {
    let dump = load_dump(dump_path, supplemental_path);

    let output = serde_json::to_string(&dump)
        .expect("Could not convert dump to JSON");

    println!("{}", output);
}

fn main() {
    let dump_arg = Arg::with_name("dump")
        .long("dump")
        .help("The location of the Roblox JSON API dump")
        .required(true)
        .takes_value(true);

    let supplemental_arg = Arg::with_name("supplemental")
        .long("supplemental")
        .help("The location of the Roblox supplementary data")
        .required(true)
        .takes_value(true);

    let matches = App::new("Rodumpster")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))

        .subcommand(SubCommand::with_name("miniwiki")
            .about("Generate a simple, single-page mini Roblox wiki")
            .arg(dump_arg.clone())
            .arg(supplemental_arg.clone()))

        .subcommand(SubCommand::with_name("megadump")
            .about("Create an API dump file with additional data")
            .arg(dump_arg.clone())
            .arg(supplemental_arg.clone()))

        .get_matches();

    match matches.subcommand() {
        ("miniwiki", command_matches) => {
            let command_matches = command_matches.unwrap();
            let dump_path = Path::new(command_matches.value_of("dump").unwrap());
            let supplemental_path = Path::new(command_matches.value_of("supplemental").unwrap());

            miniwiki(dump_path, supplemental_path);
        },
        ("megadump", command_matches) => {
            let command_matches = command_matches.unwrap();
            let dump_path = Path::new(command_matches.value_of("dump").unwrap());
            let supplemental_path = Path::new(command_matches.value_of("supplemental").unwrap());

            megadump(dump_path, supplemental_path);
        },
        _ => println!("{}", matches.usage()),
    }
}
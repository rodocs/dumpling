#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate clap;

pub mod dump;
pub mod miniwiki;
pub mod supplement;
pub mod templating;

use std::{
    collections::HashMap,
    fs,
    io,
    path::Path,
};

use clap::{
    App,
    SubCommand,
    Arg,
};

use ::{
    dump::Dump,
};

// static INSTANCE_SOURCE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Instance.md"));

fn load_dump(dump_path: &Path, supplemental_path: &Path) -> io::Result<Dump> {
    let dump_source = fs::read_to_string(dump_path)?;

    // TODO: Handle JSON errors gracefully
    let mut dump: Dump = serde_json::from_str(&dump_source)
        .expect("Could not parse dump file");

    let mut supplemental = HashMap::new();

    for entry in fs::read_dir(supplemental_path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let metadata = fs::metadata(&entry_path)?;

        // TODO: Recursive, probably
        if metadata.is_file() {
            let entry_source = fs::read_to_string(entry_path)?;

            supplement::parse(&entry_source, &mut supplemental)
                .expect("Could not parse supplemental material");
        }
    }

    for class in dump.classes.iter_mut() {
        match supplemental.get(&class.name) {
            Some(description) => {
                class.description = Some(description.prose.clone());
            },
            None => {},
        }
    }

    Ok(dump)
}

fn miniwiki(dump_path: &Path, supplemental_path: &Path) {
    let dump = load_dump(dump_path, supplemental_path)
        .expect("Could not load dump");

    let mut output = String::new();
    miniwiki::emit_dump(&dump, &mut output).unwrap();

    println!("{}", output);
}

fn megadump(dump_path: &Path, supplemental_path: &Path) {
    let dump = load_dump(dump_path, supplemental_path)
        .expect("Could not load dump");

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

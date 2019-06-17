#![recursion_limit="1024"]

mod devhub;
mod dump;
mod dump_devhub;
mod heuristics;
mod mini;
mod miniwiki;
mod reflection_metadata;
mod supplement;
mod database;

use std::{
    fs,
    path::Path,
};

use clap::{
    App,
    SubCommand,
    Arg,
};

use crate::{
    dump::{Dump, ContentSource},
    supplement::SupplementalData,
    reflection_metadata::ReflectionMetadata,
    dump_devhub::DevHubData,
    database::Database,
};

fn apply_reflection_metadata(dump: &mut Dump, metadata: &ReflectionMetadata) {
    for class in dump.classes.iter_mut() {
        match metadata.classes.get(&class.name) {
            Some(metadata_class) => {
                if metadata_class.summary.len() > 0 {
                    class.description = Some(metadata_class.summary.clone());
                }

                for member in class.members.iter_mut() {
                    if let Some(meta_member) = metadata_class.members.get(member.get_name()) {
                        if meta_member.summary.len() > 0 {
                            member.set_description(meta_member.summary.clone(), ContentSource::ReflectionMetadata);
                        }
                    }
                }
            },
            None => {},
        }
    }
}

fn apply_devhub(dump: &mut Dump, content: &DevHubData) {
    for devhub_class in content.classes.values() {
        if let Some(dump_class) = dump.classes.iter_mut().find(|item| item.name == devhub_class.name) {
            dump_class.description = Some(devhub_class.description.clone());

            for property in &devhub_class.properties {
                if let Some(dump_member) = dump_class.properties_mut().find(|item| item.name == property.name) {
                    dump_member.description = Some(property.description.clone());
                    dump_member.description_source = Some(ContentSource::DevHub);
                }
            }
        }
    }
}

fn load_database(
    dump_path: Option<&Path>,
    reflection_metadata_path: Option<&Path>,
    content_path: &Path,
) -> Database {
    let dump = Dump::read(dump_path)
        .expect("Could not load JSON API dump");

    let mut database = dump.create_database();

    let metadata = ReflectionMetadata::read(reflection_metadata_path)
        .expect("Could not load ReflectionMetadata!");

    let community_content = SupplementalData::read_from_path(content_path)
        .expect("Could not load community content");

    // TODO: ReflectionMetadata
    // TODO: Heuristics
    community_content.apply(&mut database);
    // TODO: DevHub

    database
}

struct MiniwikiOptions<'a> {
    output_path: &'a Path,
    dump_path: Option<&'a Path>,
    metadata_path: Option<&'a Path>,
    content_path: &'a Path,
}

fn miniwiki(options: &MiniwikiOptions) {
    let dump = load_database(options.dump_path, options.metadata_path, options.content_path);

    let mut output = String::new();
    mini::emit_wiki(&dump, &mut output)
        .expect("Could not generate Miniwiki");

    fs::write(options.output_path, &output)
        .expect("Could not write to output file");
}

struct MegadumpOptions<'a> {
    output_path: &'a Path,
    dump_path: Option<&'a Path>,
    metadata_path: Option<&'a Path>,
    content_path: &'a Path,
}

fn megadump(options: &MegadumpOptions) {
    let database = load_database(options.dump_path, options.metadata_path, options.content_path);

    let output = serde_json::to_string(&database)
        .expect("Could not convert database to JSON");

    fs::write(options.output_path, &output)
        .expect("Could not write to output file");
}

fn main() {
    let dump_arg = Arg::with_name("dump")
        .long("dump")
        .help("The location of the Roblox JSON API dump")
        .takes_value(true);

    let metadata_arg = Arg::with_name("metadata")
        .long("metadata")
        .help("The location of the Roblox ReflectionMetadata.xml file")
        .takes_value(true);

    let content_arg = Arg::with_name("content")
        .long("content")
        .help("The location of the Roblox supplementary data")
        .required(true)
        .takes_value(true);

    let output_arg = Arg::with_name("output")
        .long("output")
        .short("o")
        .help("Where to output the resulting file")
        .required(true)
        .takes_value(true);

    let matches = App::new("Dumpling")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))

        .subcommand(SubCommand::with_name("miniwiki")
            .about("Generate a simple, single-page mini Roblox wiki")
            .arg(dump_arg.clone())
            .arg(metadata_arg.clone())
            .arg(content_arg.clone())
            .arg(output_arg.clone()))

        .subcommand(SubCommand::with_name("megadump")
            .about("Create an API dump file with additional data")
            .arg(dump_arg.clone())
            .arg(metadata_arg.clone())
            .arg(content_arg.clone())
            .arg(output_arg.clone()))

        .get_matches();

    match matches.subcommand() {
        ("miniwiki", command_matches) => {
            let command_matches = command_matches.unwrap();
            let output_path = Path::new(command_matches.value_of("output").unwrap());
            let dump_path = command_matches.value_of("dump").map(Path::new);
            let metadata_path = command_matches.value_of("metadata").map(Path::new);
            let content_path = Path::new(command_matches.value_of("content").unwrap());

            miniwiki(&MiniwikiOptions {
                output_path,
                dump_path,
                metadata_path,
                content_path,
            });
        },
        ("megadump", command_matches) => {
            let command_matches = command_matches.unwrap();
            let output_path = Path::new(command_matches.value_of("output").unwrap());
            let dump_path = command_matches.value_of("dump").map(Path::new);
            let metadata_path = command_matches.value_of("metadata").map(Path::new);
            let content_path = Path::new(command_matches.value_of("content").unwrap());

            megadump(&MegadumpOptions {
                output_path,
                dump_path,
                metadata_path,
                content_path,
            });
        },
        _ => eprintln!("{}", matches.usage()),
    }
}
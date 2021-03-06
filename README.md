# Dumpling
Dumpling is a tool that combines data from several sources and creates an easy-to-use documentation database that anyone can use and contribute to.

Dumpling aggregates:
* Roblox Studio JSON API dump
* Roblox Studio `ReflectionMetadata.xml`
* Roblox Developer Hub (**soon!**)
* Hand-crafted heuristics, like how deprecated members are usually `camelCase`
* [Community documentation](https://github.com/rodocs/docs)

## Installation
To install Dumpling, you'll need the most recent stable version of [Rust](https://www.rust-lang.org/).

Rust was chosen for a project like this because it's fast, portable, doesn't need a scripting runtime, and has a good static type system.

Once you have Rust, you can run:

```sh
cargo install --git https://github.com/rodocs/dumpling.git
```

## Usage
Dumpling needs:
* Roblox Studio's JSON API Dump
	* Dumpling can find this automatically if you have Roblox Studio installed!
	* You can specify `--dump <file>` to pass in your own
* Roblox Studio's `ReflectionMetadata.xml` file
	* Dumpling can find this automatically, too!
	* Specify `--metadata <file>` to pass in a custom one
* A user content directory
	* **Soon**, Dumpling will pull this from [https://github.com/rodocs/docs](https://github.com/rodocs/docs)
	* Until then, use `--content content` to use the `content` directory from this repository.

Dumpling has two modes to demonstrate its functionality: Megadump, and Miniwiki.

### Megadump
Megadump generates a JSON API dump with extra information attached. You can use this as the foundation for your own API reference or other tools that want to consume API information.

```sh
cargo run -- megadump --content content -o megadump.json
```

### Miniwiki
Miniwiki generates a single page, offline-accessible, miniature API reference. It's intended as an example of the information contained in Dumpling.

```sh
cargo run -- miniwiki --content content -o miniwiki.html
```

## License
Dumpling is available under the terms of the Mozilla Public License, Version 2.0. See [LICENSE.txt](LICENSE.txt) for details.

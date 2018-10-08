# Dumpling
Dumpling is a tool to combine Roblox's JSON API dump with user-authored descriptions of APIs.

## Goals
* Aggregate:
	* Roblox JSON API dump
	* ReflectionMetadata.xml
	* Community-authored descriptions
* Produces easily consumed JSON

## Usage
Dumpling has two modes to demonstrate its functionality: Megadump, and Miniwiki.

Dumpling needs access to the Roblox JSON API dump, which can be produced via a Roblox Studio binary:

```sh
RobloxStudioBeta.exe -API dump.json
```

Dumpling also needs access to a 'supplemental content' directory. This repository includes one, located in the `supplemental` directory.

### Megadump
Megadump generates a JSON API dump with extra information attached. You can use this as the foundation for your own API reference or other tools that want to consume API information.

```sh
cargo run -- megadump --dump dump.json --supplemental supplemental > megadump.json
```

### Miniwiki
Miniwiki generates a single page, offline-capable, miniature API reference. It's intended as an example of the information contained in Dumpling.

```sh
cargo run -- miniwiki --dump dump.json --supplemental supplemental > miniwiki.html
```

## Requirements
* Rust 1.30+ (currently beta)

## License
Dumpling is available under the terms of the Mozilla Public License, Version 2.0. See [LICENSE](LICENSE) for details.
# Dumpling
Dumpling is a tool to combine Roblox's JSON API dump with user-authored descriptions of APIs.

Dumpling aggregates a bunch of data:
* Roblox JSON API dump
* ReflectionMetadata.xml
* Hand-crafted heuristics
* Community-authored descriptions

Dumpling produces easily consumed JSON in the same structure as the current Roblox JSON API dump, meaning it's pretty close to a drop-in replacement!

It can also produce a single file, offline-accesible miniature Roblox API reference that can be used to verify its content.

## Usage
Dumpling has two modes to demonstrate its functionality: Megadump, and Miniwiki.

Dumpling needs access to:

The Roblox JSON API dump, which can be produced via a Roblox Studio binary:

```sh
RobloxStudioBeta.exe -API dump.json
```

Roblox's `ReflectionMetadata.xml`, which can be pulled out of a Roblox installation

A 'supplemental content' directory. This repository includes one, located in the `supplemental` directory.

### Megadump
Megadump generates a JSON API dump with extra information attached. You can use this as the foundation for your own API reference or other tools that want to consume API information.

```sh
cargo run -- megadump --dump dump.json --metadata ReflectionMetadata.xml --supplemental supplemental > megadump.json
```

### Miniwiki
Miniwiki generates a single page, offline-capable, miniature API reference. It's intended as an example of the information contained in Dumpling.

```sh
cargo run -- miniwiki --dump dump.json --metadata ReflectionMetadata.xml --supplemental supplemental > miniwiki.html
```

## Requirements
* Rust 1.29+

## License
Dumpling is available under the terms of the Mozilla Public License, Version 2.0. See [LICENSE](LICENSE) for details.
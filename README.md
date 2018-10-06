# Rodumpster
Roblox Wiki-style data aggregation/source with no frontend.

## Goals
* Aggregator for Roblox API dump, ReflectionMetadata.xml, and human-edited descriptions
* Produces easily consumed JSON

## Usage
Rodumpster has two modes: Megadump and Miniwiki.

### Megadump
Megadump generates an API dump with extra information. You can use this as the foundation for your own wiki website or other tools that want to consume API information.

```sh
cargo run -- megadump --dump dump.json --supplemental supplemental > megadump.json
```

### Miniwiki
Miniwiki generates a single page, miniature version of a Roblox wiki. It's intended as an example of the information contained in Rodumpster.

```sh
cargo run -- miniwiki --dump dump.json --supplemental supplemental > miniwiki.html
```

## Requirements
Needs Rust 1.30+ (currently beta) and the API dump, put into `dump.json`.
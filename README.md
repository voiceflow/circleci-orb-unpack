# CircleCI Orb Unpacking Tool

[![Crates.io](https://img.shields.io/crates/v/orb-unpack)](https://crates.io/crates/orb-unpack)
[![License](https://img.shields.io/badge/license-ISC-green)](./LICENSE)

This CLI tool is designed to facilitate the process of migrating a CircleCI Orb to the Orb Development Kit format, where components are stored in separate files rather than in one `orb.yml` file. By running this tool on a given `orb.yml` file, the specified destination directory will be populated with a file structure as as follows:

```
src
├── @orb.yml
├── commands
├── executors
└── jobs
```

This output is designed to be compatible with the `circleci orb pack` command such that running it on the destination directory will produce an equivalent orb file to the original source of the unpacking.

## Installation

Ensure that you have `cargo` installed on your machine and run:

```bash
cargo install orb-unpack
```

## Usage

```bash
orb-unpack ./orb.yml ./src
```

This will create a `src/` directory in the current directory and unpack the orb's compoents into `src/`.

## License

Licensed under ISC

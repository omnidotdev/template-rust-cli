# 🦀 Rust CLI Template

This is a template repository for a command-line application built with Rust.

## Features

- 🚀 **Modern Stack**: Built with Rust 2024 edition for performance and safety
- 🛠️ **CLI Framework**: [Clap](https://docs.rs/clap) with derive macros for ergonomic argument parsing
- ⚠️ **Error Handling**: [thiserror](https://docs.rs/thiserror) for custom error types
- 🔒 **Safety**:
  - Unsafe code forbidden via lint
  - Strict Clippy lints (pedantic, nursery, cargo)
- ⚡ **Optimized Builds**:
  - LTO (Link-Time Optimization) enabled
  - Single codegen unit for maximum optimization
  - Binary stripping for smaller size
  - Panic abort for reduced binary size
- 🛠️ **Developer Experience**:
  - Comprehensive Clippy configuration
  - Cargo metadata for crates.io publishing
  - Easy spin up with [Tilt](https://tilt.dev)

## Prerequisites

- [Rust](https://rustup.rs) 1.85+

## Local Development

### Installation

```sh
cargo build
```

### Running

```sh
cargo run -- --help
```

### Testing

```sh
cargo test
```

### Linting

```sh
cargo clippy
```

## Installation

### From crates.io

```sh
cargo install {{project-name}}
```

### From source

```sh
cargo install --path .
```

## Usage

```sh
{{binary-name}} --help
```

## Publishing

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Publish to crates.io:

```sh
cargo publish
```

## License

The code in this repository is licensed under MIT, &copy; [Omni LLC](https://omni.dev). See [LICENSE.md](LICENSE.md) for more information.

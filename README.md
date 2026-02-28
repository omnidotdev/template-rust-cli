<div align="center">
  <img src="/assets/logo.png" width="100" />

  <h1 align="center">{{project-name}}</h1>

[Website](https://{{binary-name}}.omni.dev) | [Docs](https://docs.omni.dev/armory/{{project-name}}) | [Feedback](https://backfeed.omni.dev/workspaces/omni/projects/{{project-name}}) | [Discord](https://discord.gg/omnidotdev) | [X](https://x.com/omnidotdev)

</div>

**{{project-name}}** is a command-line application built with Rust.

## Installation

| Platform | Channel | Command / Link |
| --- | --- | --- |
| All | [GitHub Releases](https://github.com/omnidotdev/{{project-name}}/releases) | Download from releases page |
| All | [crates.io](https://crates.io/crates/{{project-name}}) | `cargo install {{project-name}}` |
| macOS / Linux | [Homebrew](https://github.com/omnidotdev/homebrew-tap/blob/master/Formula/{{binary-name}}.rb) | `brew install omnidotdev/tap/{{binary-name}}` |
| Arch Linux | [AUR](https://aur.archlinux.org/packages/omnidotdev-{{project-name}}) / [AUR (bin)](https://aur.archlinux.org/packages/omnidotdev-{{project-name}}-bin) | `paru -S omnidotdev-{{project-name}}` or `paru -S omnidotdev-{{project-name}}-bin` |

### Build from source

```bash
git clone https://github.com/omnidotdev/{{project-name}}
cd {{project-name}}
cargo build --release
# Binary will be at target/release/{{binary-name}}
```

## Quick Start

```sh
{{binary-name}} --help
```

## Development

### Prerequisites

- [Rust](https://rustup.rs) 1.85+
- [Bun](https://bun.sh) 1.0+

### Commands

```sh
cargo build          # Build
cargo run -- --help  # Run
cargo test           # Test
cargo clippy         # Lint
```

### Version Syncing

This project uses a dual-package setup (Rust crate + npm package) with automated version synchronization:

- **Source of truth**: `package.json` holds the canonical version, and is used for Changesets
- **Sync script**: `scripts/syncVersion.ts` propagates the version to `Cargo.toml`
- **Changesets**: Manages version bumps and changelog generation

The sync script runs automatically during the release process via the `version` npm script:

```sh
bun run version  # syncs `package.json` version → `Cargo.toml`
```

### CI/CD

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `test.yml` | Push/PR to `master` | Runs fmt, clippy, and tests |
| `sync.yml` | PR to `master` | Validates version sync, fmt, clippy, test, build |
| `release.yml` | Push to `master` | Creates releases via Changesets, builds multi-platform binaries |

### Release Process

1. Create a changeset: `bun changeset`
2. Push to `master`
3. Changesets action creates a "Version Packages" PR
4. Merge the PR to trigger a release with binaries for:
   - `x86_64-unknown-linux-gnu`
   - `aarch64-unknown-linux-gnu`
   - `x86_64-apple-darwin`
   - `aarch64-apple-darwin`
5. **Manually** publish to crates.io: `cargo publish`

## Ecosystem

- **[Omni CLI](https://github.com/omnidotdev/cli)**: Agentic CLI for the Omni ecosystem
- **[Omni Terminal](https://github.com/omnidotdev/terminal)**: GPU-accelerated terminal emulator built to run everywhere

## License

The code in this repository is licensed under MIT, &copy; [Omni LLC](https://omni.dev). See [LICENSE.md](LICENSE.md) for more information.

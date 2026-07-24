# 🗺️ Cargo QC Roadmap

This document outlines the development phases to elevate `cargo-qc` to a professional `crates.io` standard. The golden rule is to complete every milestone in a version before moving to the next.

## v0.4.0: Architecture and UX Foundations
- [ ] Architecture Refactoring: Separate the code into `src/lib.rs` (logic) and `src/main.rs` (CLI wrapper).
- [ ] Implement advanced error handling using `anyhow` and `thiserror`.
- [ ] Integrate the `clap` crate for command-line argument parsing (e.g., `--skip-fmt`, `--ci` flags).
- [ ] Add official support for executing `cargo test` in the pipeline (alongside fmt, clippy, and build).

## v0.5.0: Aesthetics and Visual Experience
- [ ] Implement the `indicatif` crate to add animated progress spinners in the terminal.
- [ ] Ensure strict compatibility with the `NO_COLOR` environment variable.
- [ ] Improve user-facing error messages to be more descriptive and clean.

## v0.6.0: Configuration and Testing
- [ ] Support an optional configuration file (`cargo-qc.toml`) allowing users to persistently enable/disable specific checks in their repositories.
- [ ] Implement Integration Tests using the `assert_cmd` crate to simulate CLI execution.
- [ ] Add Unit Tests for the core logic in `lib.rs`.

## v0.7.0: Open Source Standardization
- [ ] Implement tools for automated `CHANGELOG.md` generation (e.g., `git-cliff`).
- [ ] Create a `CONTRIBUTING.md` document with clear guidelines for contributing to the project.
- [ ] Update the `README.md` by adding a demonstration GIF or animated SVG (`vhs`) showing the tool in action.
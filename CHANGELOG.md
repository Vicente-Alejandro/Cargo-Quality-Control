# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### Bug Fixes

- Re-apply dora metrics on top of v0.6.0

### Documentation

- Bump version in README to 0.6.6
- Translate v0.6.x tasks to english and mark as completed
- Update terminal output to match v0.6.4 format
- Redefine v0.6.x to focus on strict maintainability and continuous auditing

### Features

- Add background thread to warn users if the process exceeds 15 seconds
- Make cargo fmt failure hint more explicit about auto-fixing
- Add continuous auditing via webhook
- Add strict maintainability mode without configuration file

### Miscellaneous Tasks

- Bump version to 0.6.6
- Bump version to 0.6.5
- Bump version to 0.6.4
- Add assert_cmd and predicates as dev-dependencies

### Refactor

- Modularize architecture for v0.6.3

### Styling

- Format long text lines recently added
- Run cargo fmt to fix unformatted code generated during testing

### Testing

- Add critical boundary tests for skip flags, telemetry, and env vars
- Add integration tests using assert_cmd

## [0.5.1] - 2026-07-26

### Documentation

- Update terminal output examples for v0.5 UI changes
- Mark v0.5.0 as completed in ROADMAP

## [0.5.0] - 2026-07-26

### Features

- Native zero-dependency UI aesthetics

## [0.4.8] - 2026-07-26

### Features

- [**breaking**] Drop time formatting and parsing to remove macro deps
- [**breaking**] Drop clap and thiserror for ultra-minimal footprint
- [**breaking**] Replace chrono/dialoguer/colored with minimal alternatives

### Miscellaneous Tasks

- Bump version to v0.4.8

## [0.4.6] - 2026-07-25

### Miscellaneous Tasks

- Explicitly link documentation field in Cargo.toml

## [0.4.5] - 2026-07-25

### Documentation

- Fix rustdoc warnings by marking README codeblocks as text

## [0.4.4] - 2026-07-25

### Documentation

- Add rustdoc comments for public items

## [0.4.3] - 2026-07-25

### Documentation

- Update README terminal output examples to include cargo test

## [0.4.2] - 2026-07-24

### Miscellaneous Tasks

- Add missing crates.io metadata to Cargo.toml
- Commit Cargo.lock

## [0.4.1] - 2026-07-24

### Documentation

- Document new CLI arguments and flags in README

## [0.4.0] - 2026-07-24

### Documentation

- Bump version to 0.4.0 and update roadmap status

### Features

- Add cargo test support to quality gates pipeline
- Add CLI arguments support using clap

### Refactor

- Extract logic into lib.rs and implement proper error handling

## [0.3.1] - 2026-07-24

### Bug Fixes

- Correctly parse cargo pkgid version strings

### Documentation

- Translate ROADMAP.md to English and remove deferred CI steps
- Add ROADMAP.md for future development phases

## [0.3.0] - 2026-07-24

### Documentation

- Update version to 0.3.0

### Features

- Add interactive prompt to ignore cargo-qc directory

### Miscellaneous Tasks

- Add project metadata to Cargo.toml

## [0.2.0] - 2026-07-24

### Miscellaneous Tasks

- Bump version to 0.2.0 and update README

## [0.1.0] - 2026-07-24

### Documentation

- Add README.md and update gitignore

### Refactor

- Resolve clippy warnings, format code, and standardize log directory

### Build

- Add colored and chrono dependencies



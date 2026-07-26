# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.8] - 2026-07-25

### Changed
- **Zero-config & Ultra-minimal:** Radically redesigned the dependency tree to achieve the absolute minimum footprint for a robust Rust CLI.
- Replaced `dialoguer` with manual `std::io::stdin` parsing for interactive prompts.
- Replaced `colored` with `owo-colors` for zero-dependency ANSI terminal coloring.
- Replaced `chrono` with `time` (stripped down to just system time, dropping parsing and formatting features) to eliminate calendar and macro overhead.
- Removed `clap` completely and hand-rolled `std::env::args` parsing to eliminate the heavy `syn`/`quote`/`proc-macro2` compiler ecosystem.
- Removed `thiserror` completely and manually implemented `std::fmt::Display` and `std::error::Error` for the main error type.
- **Performance:** Cold compilation time was reduced from ~2 minutes to <20 seconds. Total compiled dependency crates reduced from 63 to 9 (an 85% reduction).

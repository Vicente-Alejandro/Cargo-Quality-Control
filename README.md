# Cargo Quality Control (cargo-qc)

🚀 A dead-simple local quality control runner for Rust projects.

`cargo-qc` provides an easy way to run `cargo fmt`, `cargo clippy`, and `cargo build` sequentially and logs the results (successes and errors) directly into a standardized directory inside your project. This creates an auditable history of the code quality over time.

## Features

- **Automated Checks**: Runs `fmt --check`, `clippy -D warnings`, and `build` in one go.
- **Traceability**: Automatically records the history of runs inside `.qc_history.md`.
- **Error Logging**: Saves any linting or build errors to `.qc_errors.log` for easy reference.
- **Project Agnostic**: Can be run from any Rust project. It will automatically detect the crate version and create a `/tools/cargo-qc` directory at the root of the project to store the logs.

## Installation

You can install `cargo-qc` locally using cargo:

```bash
cargo install --path .
```
*(Once published, you will be able to install it via `cargo install cargo-qc`)*

## Usage

Simply navigate to your Rust project directory and run:

```bash
cargo qc
```

(Or, if not installed globally, you can execute its binary directly via `cargo run`).

### What happens when you run it?

1. It determines the current project version using `cargo pkgid`.
2. It ensures the `tools/cargo-qc` directory exists in the current project.
3. It runs `cargo fmt -- --check`.
4. It runs `cargo clippy -- -D warnings`.
5. It runs `cargo build`.
6. It appends the results (✅ or ❌) to `tools/cargo-qc/.qc_history.md` as a Markdown table.
7. If any checks fail, it saves the detailed output to `tools/cargo-qc/.qc_errors.log` and exits with an error code.

## Log Output Examples

**`.qc_history.md`**

| Date | Version | Fmt | Clippy | Build | Overall |
|---|---|---|---|---|---|
| 2026-07-24 01:22 | 0.1.0 | ❌ | ❌ | ✅ | ❌ |
| 2026-07-24 01:25 | 0.1.0 | ✅ | ✅ | ✅ | ✅ |

**`.qc_errors.log`**

```text
========================================
DATE: 2026-07-24 01:22 | VERSION: 0.1.0
========================================
--- FMT ERROR ---
...
```

## Contributing

Contributions are welcome! Please run `cargo qc` on your own code before opening a PR to ensure that all checks pass.

## License

MIT License

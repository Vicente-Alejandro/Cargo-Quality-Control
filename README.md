<div align="center">

# Cargo Quality Control v0.4.0

**Local quality control automation for Rust projects.**

[![Crates.io](https://img.shields.io/crates/v/cargo-qc?style=flat-square&color=orange)](https://crates.io/crates/cargo-qc)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

`cargo fmt` · `cargo clippy` · `cargo build` — in one command, with an auditable log history.

</div>

---

## Overview

`cargo-qc` is a zero-configuration Cargo subcommand that runs the three standard Rust quality gates in sequence and records every result into a structured, version-tagged log inside your project.

It solves a specific problem: ensuring that **no commit ever skips a quality check**, and that the history of those checks is **readable, auditable, and co-located with the code** — not locked inside a CI provider's dashboard.

```
$ cargo qc

[cargo-qc] Project: Bevy v0.19.1
[cargo-qc] Running cargo fmt --check ...   ✅
[cargo-qc] Running cargo clippy ...        ✅
[cargo-qc] Running cargo build ...         ✅
[cargo-qc] All checks passed. Log written to tools/cargo-qc/.qc_history.md
```

---

## Table of Contents

- [Why cargo-qc](#why-cargo-qc)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
- [Output & Logs](#output--logs)
- [CI Integration](#ci-integration)
- [Exit Codes](#exit-codes)
- [Contributing](#contributing)
- [License](#license)

---

## Why cargo-qc

Running `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo build` individually before every commit is easy to skip under time pressure. Relying solely on CI means broken quality gates are caught after the push, not before.

`cargo-qc` gives you:

- **Pre-commit discipline** — a single command to run all three gates locally, in the correct order, with a consistent set of flags.
- **Auditable history** — every run is appended to a Markdown table with timestamp and crate version. The file lives in your repository, travels with the code, and survives CI provider changes.
- **Immediate error visibility** — failures are saved to a structured log file with the exact compiler output, separated by check type. No scrolling through interleaved terminal output.
- **Zero configuration** — works in any Rust project without setup. No config files, no environment variables.

---

## Requirements

| Requirement | Minimum version |
|---|---|
| Rust toolchain | 1.70.0 |
| Cargo | ships with Rust |
| Operating system | Linux · macOS · Windows |

The `clippy` and `rustfmt` components must be installed. If they are missing, add them with:

```bash
rustup component add clippy rustfmt
```

---

## Installation

**From source (local development):**

```bash
git clone https://github.com/Vicente-Alejandro/cargo-qc
cd cargo-qc
cargo install --path .
```

**From crates.io (once published):**

```bash
cargo install cargo-qc
```

After installation, `cargo-qc` is available as a Cargo subcommand in any directory:

```bash
cargo qc
```

---

## Usage

Navigate to the root of any Rust project and run:

```bash
cargo qc
```

### What happens, step by step

| Step | Command | Flags |
|---|---|---|
| 1 | Detect project version | `cargo pkgid` |
| 2 | Ensure log directory exists | creates `tools/cargo-qc/` if absent |
| 3 | Check formatting | `cargo fmt -- --check` |
| 4 | Run linter | `cargo clippy -- -D warnings` |
| 5 | Compile | `cargo build` |
| 6 | Run tests | `cargo test` |
| 7 | Append result row | writes to `tools/cargo-qc/.qc_history.md` |
| 8 | On failure: save errors | writes to `tools/cargo-qc/.qc_errors.log` |

Steps 3–6 run sequentially. **A failing step does not abort subsequent steps** — all checks run regardless, so you get a complete picture of the project's current state in a single pass.

If any step fails, `cargo-qc` exits with a non-zero exit code after writing the error log.

### Ignoring `tools/cargo-qc` from Git

The log directory is designed to be committed alongside your code for auditability. If you prefer to exclude it, add the following to your `.gitignore`:

```gitignore
# Uncomment to exclude cargo-qc logs from version control
# tools/cargo-qc/
```

---

## Output & Logs

### Terminal output

```
[cargo-qc] Project: my-project v1.2.0
[cargo-qc] Running cargo fmt --check ...   ✅
[cargo-qc] Running cargo clippy ...        ❌
[cargo-qc] Running cargo build ...         ✅
[cargo-qc] 1 check(s) failed. See tools/cargo-qc/.qc_errors.log for details.
```

### `tools/cargo-qc/.qc_history.md`

A persistent Markdown table, one row per run, appended automatically:

| Date | Version | Fmt | Clippy | Build | Test | Overall |
|------|---------|-----|--------|-------|------|---------|
| 2026-07-24 01:15 | 0.4.0 | ❌ | ❌ | ✅ | ✅ | ❌ |
| 2026-07-24 01:22 | 0.4.0 | ✅ | ❌ | ✅ | ✅ | ❌ |
| 2026-07-24 01:31 | 0.4.0 | ✅ | ✅ | ✅ | ✅ | ✅ |

### `tools/cargo-qc/.qc_errors.log`

Structured error output, written only when at least one check fails. Previous error content is **overwritten** on each run — the file always reflects the most recent failure:

```
========================================
DATE: 2026-07-24 01:22 | VERSION: 0.3.1
========================================

--- CLIPPY ERRORS ---
error: unused variable: `config`
  --> src/main.rs:42:9
   |
42 |     let config = load_config();
   |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_config`
   |
   = note: `-D unused-variables` implied by `-D warnings`
```

---

## CI Integration

`cargo-qc` exits with a non-zero code on any failure, making it compatible with any CI system that checks exit codes.

### GitHub Actions

```yaml
# .github/workflows/qc.yml
name: Quality Control

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  quality-control:
    name: cargo-qc
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt

      - name: Install cargo-qc
        run: cargo install --path tools/cargo-qc
        # Or: cargo install cargo-qc (once published)

      - name: Run quality checks
        run: cargo qc

      - name: Upload QC history
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: qc-logs
          path: tools/cargo-qc/
```

> **Note:** When running in CI, the `tools/cargo-qc/.qc_history.md` artifact captures the run history even if the checks fail, giving reviewers full context on the PR.

---

## Exit Codes

| Code | Meaning |
|---|---|
| `0` | All checks passed |
| `1` | One or more checks failed (see `.qc_errors.log`) |
| `2` | Internal error — could not detect project version or create log directory |

---

## Contributing

Contributions are welcome. Before opening a pull request:

1. **Run `cargo qc`** on your changes. All three checks must pass.
2. **Follow the existing code style** — `cargo fmt` is enforced.
3. **Keep changes focused** — one logical change per PR. See the project's commit conventions for guidance.
4. **Update documentation** if your change affects behavior, flags, or output format.

For significant changes, open an issue first to discuss the approach before implementing.

---

## License

`cargo-qc` is released under the [MIT License](./LICENSE).
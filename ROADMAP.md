# 🗺️ Cargo QC Roadmap

This document outlines the development phases to elevate `cargo-qc` to a professional `crates.io` standard. The golden rule is unchanged: **complete every milestone in a version before moving to the next.** Each milestone below adds a short "Definition of Done" so that rule is checkable, not just aspirational.

---

## v0.4.0: Architecture and UX Foundations

- [x] **Architecture refactoring.** Split into `src/lib.rs` (logic, organized into `checks.rs`, `logging.rs`, `config.rs` modules) and a thin `src/main.rs` that only parses arguments, calls the library, and translates the result into an exit code. This is what makes v0.6.0's unit tests possible — a `main.rs`-only implementation can't be unit tested without spawning a real process.
- [x] **Structured error handling.** Define a `QcError` enum in `lib.rs` with `thiserror`, one variant per failure domain (`VersionDetection`, `LogDirCreation(#[from] io::Error)`, `CheckFailed { check: CheckKind }`, `ConfigParse(#[from] toml::de::Error)`, …), each preserving its source via `#[from]`/`#[source]`. Keep `anyhow` in `main.rs` only, where it's used to add human-readable `.context(...)` breadcrumbs and unify the top-level `Result` before mapping it back to one of the three documented exit codes. This is the standard split for a Rust CLI: `thiserror` for the library's typed, matchable errors; `anyhow` for the application layer that just needs to report what went wrong.
- [x] **`clap` integration** (derive API, `#[derive(Parser)]` / `#[derive(Subcommand)]`, `#[command(author, version, about, long_about = None)]`):
  - `--skip-fmt`, `--skip-clippy`, `--skip-build`, `--skip-test` — opt out of individual checks.
  - `--ci` — non-interactive mode: never prompts for the `.gitignore` decision (defaults to "yes, ignore"), and keeps terminal output linear (see v0.5.0 on spinners in CI).
  - `--config <PATH>` — override the config file location (ties into v0.6.0).
  - `-q` / `--quiet`.
- [x] **`cargo test` as a fourth check**, alongside fmt/clippy/build, following the existing "run everything regardless of earlier failures" philosophy — extend the history table to `Fmt | Clippy | Build | Test | Overall` and expose `--skip-test` for workspaces where the test suite is slow or needs external services.

**Definition of Done:** `cargo-qc` still passes its own `cargo qc` after the refactor; the four checks all run and log correctly; `--help` documents every flag above.

---

## v0.5.0: Aesthetics and Visual Experience (Ultra-Slim Edition)

- [x] **Zero-Dependency Spinners.** Implemented a native threaded spinner using `std::thread` and `std::sync::atomic::AtomicBool` to animate the loading state. Bypasses `indicatif` to maintain the < 20s compile time. Gracefully falls back to plain text if `stderr` is not a TTY (or when `--ci` is used).
- [x] **`NO_COLOR` compliance.** Added a native `--no-color` flag and detection for the `NO_COLOR` environment variable to explicitly strip out `owo-colors` styling, ensuring the output is strictly plain text when required.
- [x] **Sharper error messages.** Truncated very long stderr blocks in the terminal view to avoid blowing out the screen, while keeping the full text in `.qc_errors.log`. Added actionable "what to run next" hints for standard check failures (e.g. suggesting `cargo clippy --fix`).

**Definition of Done:** Running with `NO_COLOR=1` produces byte-for-byte plain output; native spinners render smoothly in an interactive terminal and disappear cleanly in a piped/CI context. All achieved with zero new dependencies.

---

## v0.6.0: Strict Maintainability (SIG Principles) without Configuration

- [x] **Strict Maintainability (SIG Principles).** Integrate the 10 maintainability principles from *Building Maintainable Software* while respecting the "zero config" philosophy.
- [x] **`--strict` Flag.** Add an explicit `--strict` flag (and `QC_STRICT=1` env var). If strict mode is activated, `cargo-qc` will automatically inject additional flags to the `clippy` subprocess (`-D clippy::cognitive_complexity`, `-D clippy::too_many_arguments`, `-D clippy::type_complexity`). No configuration file will be created.

**Definition of Done:** `cargo qc --strict` correctly injects restrictive clippy rules, causing complex code to fail validation.

---

## v0.6.1: Delivery Metrics (DORA Metrics)

- [x] **Duration Logging.** Log the execution time of tests and checks in `.qc_history.md`, serving as a local database to calculate the developer's *Lead Time* and *Change Failure Rate* (*Accelerate*).

**Definition of Done:** `.qc_history.md` includes time/duration columns for the execution.

---

## v0.6.2: Continuous Auditing and Telemetry

- [x] **Continuous Auditing.** Move from isolated auditing to continuous auditing (*EBSCO / CISA*). Introduce a hidden `--telemetry <URL>` mode. After finishing the execution, an asynchronous JSON payload will be sent to the specified server with the run's result.

**Definition of Done:** Running `cargo qc --telemetry http://localhost:8080` sends a successful POST JSON payload with the execution report.

---

## v0.6.3: Refactoring and Resilience (QA Phase 1)

- [x] **Modular Architecture.** Split `lib.rs` into modules (`config.rs`, `history.rs`, `runner.rs`, `telemetry.rs`) to comply with SIG principles (short code units and small interfaces).
- [x] **Resilience.** Safe error handling for reading `stdin` and validation of `curl`'s existence before attempting telemetry.

**Definition of Done:** `cargo qc` works identically, but internally the code is modularized and does not produce `panics` in non-TTY environments.

---

## v0.6.4: Test Coverage (QA Phase 2)

- [x] **Integration Tests.** Configure the `assert_cmd` crate as a `dev-dependency` to simulate console commands and test output scenarios, failures, and telemetry.
- [x] **Unit Tests.** Tests for parsing, utilities, and configuration.

**Definition of Done:** A test suite exists in `tests/` that passes successfully with `cargo test`.

---

## v0.7.0: Open Source Standardization

- [x] **`git-cliff` + `cliff.toml`** with `conventional_commits = true` and `commit_parsers` grouping (`feat`, `fix`, `docs`, `refactor`, `test`, `chore`) for automated `CHANGELOG.md` generation. This implies adopting Conventional Commits going forward, which lines up with your `atomic-commits` and `semver-tagging` conventions.
- [x] **`CONTRIBUTING.md`**: PR checklist (`cargo qc` must pass locally before opening a PR), commit-message conventions, instructions for running the new integration test suite.
- [x] **`SECURITY.md`**: where to report a vulnerability, expected response window, and the supported version range — this pairs naturally with the MSRV policy from v1.0.0 below.
- [x] **`CODE_OF_CONDUCT.md`** (Contributor Covenant is the de facto standard). Low effort, and GitHub surfaces it as one of its "community standards" checks.
- [x] **README demo**: commit the `vhs` tape script (`demo.tape`) itself, not just the rendered GIF/SVG — anyone can regenerate it from source instead of trusting a stale binary asset.

**Definition of Done:** GitHub's "Community Standards" checklist for the repo is fully green; `CHANGELOG.md` regenerates cleanly from `git cliff` with no manual edits needed for the next tagged release.

---

## v0.8.0 (new): Supply Chain and CI Hardening

The README already promises Linux · macOS · Windows support — CI should actually prove that on every push, not just assert it in prose.

- [ ] **Full OS matrix** in GitHub Actions (`ubuntu-latest`, `macos-latest`, `windows-latest`) using `dtolnay/rust-toolchain@stable` + `Swatinem/rust-cache@v2` (caches `~/.cargo` and `target/`, keyed on `Cargo.lock` — this is the standard caching action for Rust CI and typically cuts build time by 50–80%).
- [ ] **`cargo audit`** (RustSec advisory database) and **`cargo deny check`** (license policy + duplicate-dependency detection) as required CI jobs — and also scheduled to run on a cron (daily or weekly), so a newly-disclosed CVE in an existing dependency surfaces even without new commits.
- [ ] **Dependabot or Renovate** configuration for automated dependency-update PRs, gated by the CI jobs above.
- [ ] **Pin third-party Actions to a commit SHA**, not a tag, in any workflow that will eventually touch `secrets.CARGO_REGISTRY_TOKEN` (v0.9.0) — standard supply-chain hardening once secrets are in play.

**Definition of Done:** a PR that only touches Windows-specific behavior is caught by CI without a human needing to test it manually; `cargo audit`/`cargo deny` are required status checks on `main`.

---

## v0.9.0 (new): Release Automation and Distribution

- [ ] **`release-plz`**: automates the `Cargo.toml` version bump and `CHANGELOG.md` update as a standing "Release PR" derived from Conventional Commit history (built directly on v0.7.0's `git-cliff` setup). Merging that PR is what triggers the actual `cargo publish`.
- [ ] **`cargo-dist`**: `dist init` generates a `release.yml` that builds prebuilt binaries for Linux/macOS/Windows and attaches them to the GitHub Release — this removes the "clone the repo and `cargo install --path .`" friction for anyone evaluating the tool without a full Rust toolchain.
- [ ] **crates.io metadata audit**: confirm `license`, `repository`, `description`, `keywords`, and — currently missing — `categories` are all set in `Cargo.toml` (crates.io uses `categories` for discovery). Add `cargo publish --dry-run` as a required, non-optional CI gate before any version tag is pushed.

**Definition of Done:** pushing a tag results in a crates.io release, a GitHub Release with attached binaries, and an updated changelog — with no manual steps beyond merging the release PR.

---

## v1.0.0 (new): Stability and Governance

Reaching `1.0.0` is itself a statement: it commits to the stability guarantees below, not just a version bump.

- [ ] **Explicit MSRV policy**, documented in the README (e.g. "an MSRV bump is a minor version, never silent") — resolves the same class of problem flagged in the housekeeping note.
- [ ] **SemVer commitment extended to the CLI surface**, not just the (now-public) library API: a breaking change to `cargo-qc.toml`'s schema or to `.qc_history.md`'s column layout is a breaking change to the tool, and should require a major version bump like any other public API change.
- [ ] **`cargo-semver-checks`** in CI, guarding the `lib.rs` public API introduced in v0.4.0 against accidental breaking changes.
- [ ] **docs.rs verification badge** and `#![warn(missing_docs)]` on the library crate, now that `lib.rs` is a real, documented public surface rather than an implementation detail of the binary.

**Definition of Done:** `cargo-semver-checks` is a required CI job; the README states an MSRV policy and a SemVer commitment in plain language, not just a version number.
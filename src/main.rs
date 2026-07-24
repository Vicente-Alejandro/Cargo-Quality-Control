use chrono::Local;
use colored::*;
use dialoguer::{Select, theme::ColorfulTheme};
use std::env;
use std::fmt::Display;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::process::Command;

/// Fixed prefix printed at the start of every informational line.
///
/// This matches the terminal-output examples already documented in
/// README.md, so the tool's real behavior finally matches what the
/// docs promise instead of drifting from it.
const PREFIX: &str = "[cargo-qc]";

/// Column width used to right-align the pass/fail icon of every check
/// line, so a run with several checks renders as one clean vertical
/// column of icons instead of a ragged list. 34 comfortably fits the
/// longest current label ("Running cargo fmt --check ...", 29 chars)
/// plus a visual gap; bump this if a future check gets a longer label.
const CHECK_LABEL_WIDTH: usize = 34;

/// Prints a standard `[cargo-qc]` informational line.
///
/// Centralizing this in one place means the prefix's styling (currently
/// dimmed, so the message itself stays the visual focus) only has to
/// change in one place if that ever needs to change. Accepts anything
/// `Display`, so both plain `&str`/`String` and `colored::ColoredString`
/// values can be passed in without an extra `.to_string()` call.
fn log_line(message: impl Display) {
    println!("{} {}", PREFIX.dimmed(), message);
}

/// Prints one quality-gate result line, e.g.:
/// `[cargo-qc] Running cargo fmt --check ...        ✅`
///
/// ✅ / ❌ are reserved exclusively for the pass/fail status of the three
/// quality gates (fmt, clippy, build) — that's the meaning README.md's
/// example output already assigns them, so we don't reuse them for
/// anything else (no decorative emoji elsewhere in this tool).
fn log_check(label: &str, passed: bool) {
    let icon = if passed { "✅" } else { "❌" };
    println!("{} {label:<CHECK_LABEL_WIDTH$}{icon}", PREFIX.dimmed());
}

/// Prints a fatal, non-check error using the same `error:` convention
/// `rustc`/`cargo`/`clippy` already use, so cargo-qc's own diagnostics
/// read consistently with the tools it wraps.
fn log_error(message: impl Display) {
    eprintln!("{} {message}", "error:".red().bold());
}

/// Prints a non-fatal warning using the same `warning:` convention as
/// `rustc`/`cargo`/`clippy`.
fn log_warning(message: impl Display) {
    println!("{} {message}", "warning:".yellow().bold());
}

fn main() {
    log_line("Local Quality Control".bold());

    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Extract Version
    let mut version = String::from("unknown");
    if let Ok(output) = Command::new("cargo").arg("pkgid").output()
        && output.status.success()
    {
        let pkgid = String::from_utf8_lossy(&output.stdout);
        if let Some((_, v)) = pkgid.rsplit_once('@') {
            version = v.trim().to_string();
        } else if let Some((_, v)) = pkgid.rsplit_once('#') {
            version = v.trim().to_string();
        }
    }

    // NOTE: this is a display-only stand-in for the crate name, taken from
    // the current directory. `cargo pkgid` *does* carry the real name, but
    // it's only present in the pkgid string when it differs from the
    // directory name — extracting it robustly is exactly the kind of logic
    // that belongs in `lib.rs` once the v0.4.0 architecture refactor lands,
    // with a unit test covering both pkgid formats. Treat this line as a
    // placeholder, not a parser, until then.
    let project_name = current_dir
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "project".to_string());

    log_line(format!(
        "Project: {} v{}",
        project_name.bold(),
        version.cyan()
    ));
    log_line(format!("Directory: {}", current_dir.display()));

    // Ensure log directory exists
    let log_dir = current_dir.join("tools").join("cargo-qc");
    if !log_dir.exists()
        && let Err(e) = std::fs::create_dir_all(&log_dir)
    {
        log_error(format!("could not create log directory: {e}"));
        std::process::exit(2);
    }

    // Check if tools/cargo-qc is in .gitignore
    let gitignore_path = current_dir.join(".gitignore");
    let skip_prompt_path = log_dir.join(".skip_gitignore_prompt");

    if !skip_prompt_path.exists() {
        let mut needs_ignore = true;
        if gitignore_path.exists()
            && let Ok(mut file) = fs::File::open(&gitignore_path)
        {
            let mut content = String::new();
            let _ = file.read_to_string(&mut content);
            if content.contains("tools/cargo-qc") {
                needs_ignore = false;
            }
        }

        if needs_ignore {
            log_warning("tools/cargo-qc/ is not in your .gitignore yet.");
            let selections = &["Yes, add it to .gitignore", "No, let me track it"];

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Automatically ignore the cargo-qc log directory?")
                .default(0)
                .items(&selections[..])
                .interact_opt()
                .unwrap_or(None);

            match selection {
                Some(0) => {
                    if let Ok(mut file) = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&gitignore_path)
                    {
                        let _ = writeln!(file, "\n# cargo-qc logs\ntools/cargo-qc/");
                        log_line("Added tools/cargo-qc/ to .gitignore.");
                    }
                }
                Some(1) => {
                    let _ = fs::File::create(&skip_prompt_path);
                    log_line("Understood — this won't be asked again.");
                }
                _ => {
                    log_line("Skipped — this will be asked again next run.");
                }
            }
        }
    }

    let history_file = log_dir.join(".qc_history.md");
    let errors_file = log_dir.join(".qc_errors.log");

    // Ensure history file has header
    if !history_file.exists()
        && let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&history_file)
    {
        let _ = writeln!(file, "# Cargo QC History\n");
        let _ = writeln!(file, "| Date | Version | Fmt | Clippy | Build | Overall |");
        let _ = writeln!(file, "|---|---|---|---|---|---|");
    }

    let mut err_log = String::new();

    // 1. Formatting
    let fmt_output = Command::new("cargo")
        .arg("fmt")
        .arg("--")
        .arg("--check")
        .output()
        .expect("Failed to execute cargo fmt");
    let fmt_pass = fmt_output.status.success();
    log_check("Running cargo fmt --check ...", fmt_pass);
    if !fmt_pass {
        err_log.push_str("--- FMT ERROR ---\n");
        err_log.push_str(&String::from_utf8_lossy(&fmt_output.stdout));
        err_log.push_str(&String::from_utf8_lossy(&fmt_output.stderr));
        err_log.push('\n');
    }

    // 2. Clippy
    let clippy_output = Command::new("cargo")
        .arg("clippy")
        .arg("--")
        .arg("-D")
        .arg("warnings")
        .output()
        .expect("Failed to execute cargo clippy");
    let clippy_pass = clippy_output.status.success();
    log_check("Running cargo clippy ...", clippy_pass);
    if !clippy_pass {
        err_log.push_str("--- CLIPPY ERROR ---\n");
        err_log.push_str(&String::from_utf8_lossy(&clippy_output.stdout));
        err_log.push_str(&String::from_utf8_lossy(&clippy_output.stderr));
        err_log.push('\n');
        // Still surface the raw compiler output immediately so the user
        // doesn't have to open the log file to start fixing lints.
        eprint!("{}", String::from_utf8_lossy(&clippy_output.stderr));
    }

    // 3. Build
    let build_output = Command::new("cargo")
        .arg("build")
        .output()
        .expect("Failed to execute cargo build");
    let build_pass = build_output.status.success();
    log_check("Running cargo build ...", build_pass);
    if !build_pass {
        err_log.push_str("--- BUILD ERROR ---\n");
        err_log.push_str(&String::from_utf8_lossy(&build_output.stdout));
        err_log.push_str(&String::from_utf8_lossy(&build_output.stderr));
        err_log.push('\n');
        eprint!("{}", String::from_utf8_lossy(&build_output.stderr));
    }

    let all_passed = fmt_pass && clippy_pass && build_pass;
    let failed_count = [fmt_pass, clippy_pass, build_pass]
        .iter()
        .filter(|passed| !**passed)
        .count();

    // Write History
    let date = Local::now().format("%Y-%m-%d %H:%M").to_string();
    let icon = |pass: bool| if pass { "✅" } else { "❌" };

    if let Ok(mut file) = OpenOptions::new().append(true).open(&history_file) {
        let _ = writeln!(
            file,
            "| {} | `{}` | {} | {} | {} | {} |",
            date,
            version,
            icon(fmt_pass),
            icon(clippy_pass),
            icon(build_pass),
            icon(all_passed)
        );
    }

    // Write errors, if any, and report the outcome.
    if !all_passed {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&errors_file)
        {
            let _ = writeln!(file, "========================================");
            let _ = writeln!(file, "DATE: {date} | VERSION: {version}");
            let _ = writeln!(file, "========================================");
            let _ = writeln!(file, "{err_log}");
        }
        log_line(
            format!(
                "{failed_count} check(s) failed. See tools/cargo-qc/.qc_errors.log for details."
            )
            .red()
            .bold(),
        );
        std::process::exit(1);
    }

    log_line(
        "All checks passed. Log written to tools/cargo-qc/.qc_history.md"
            .green()
            .bold(),
    );
}

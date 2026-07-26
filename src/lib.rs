#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// Error definitions for the cargo-qc pipeline.
pub mod error;
/// Terminal spinner implementation.
pub mod spinner;

use error::QcError;
use spinner::Spinner;
use owo_colors::OwoColorize;
use std::env;
use std::fmt::Display;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Read, Write};
use std::process::Command;
use time::OffsetDateTime;

const PREFIX: &str = "[cargo-qc]";
const CHECK_LABEL_WIDTH: usize = 34;

static mut GLOBAL_NO_COLOR: bool = false;

/// Sets the global no-color flag for logging functions.
fn set_no_color(no_color: bool) {
    unsafe {
        GLOBAL_NO_COLOR = no_color;
    }
}

fn no_color() -> bool {
    unsafe { GLOBAL_NO_COLOR }
}

/// Logs a plain message to the console with the `[cargo-qc]` prefix.
pub fn log_line(message: impl Display) {
    if no_color() {
        println!("{} {}", PREFIX, message);
    } else {
        println!("{} {}", PREFIX.dimmed(), message);
    }
}

/// Logs a check step (e.g. fmt, clippy) and whether it passed (`✅` or `❌`).
pub fn log_check(label: &str, passed: bool) {
    let icon = if passed { "✅" } else { "❌" };
    if no_color() {
        println!("{} {label:<CHECK_LABEL_WIDTH$}{icon}", PREFIX);
    } else {
        println!("{} {label:<CHECK_LABEL_WIDTH$}{icon}", PREFIX.dimmed());
    }
}

/// Logs a fatal error message in red.
pub fn log_error(message: impl Display) {
    if no_color() {
        eprintln!("error: {message}");
    } else {
        eprintln!("{} {message}", "error:".red().bold());
    }
}

/// Logs a warning message in yellow.
pub fn log_warning(message: impl Display) {
    if no_color() {
        println!("warning: {message}");
    } else {
        println!("{} {message}", "warning:".yellow().bold());
    }
}

/// Prompts the user with a `[Y/n]` question on stdout.
///
/// Accepted inputs:
/// - `y`, `Y`, a single space `' '`, or an empty Enter → returns `true` (yes)
/// - `n` or `N` → returns `false` (no)
/// - Any other input → re-prompts until a valid answer is given
fn prompt_yes_no(question: &str) -> bool {
    let stdin = io::stdin();
    loop {
        if no_color() {
            print!("{} {} [Y/n]: ", PREFIX, question);
        } else {
            print!("{} {} [Y/n]: ", PREFIX.dimmed(), question);
        }
        io::stdout().flush().expect("failed to flush stdout");

        let mut line = String::new();
        stdin
            .lock()
            .read_line(&mut line)
            .expect("failed to read stdin");

        // Trim the trailing newline/carriage-return but preserve a lone space.
        let trimmed = line.trim_end_matches(['\n', '\r']);

        match trimmed {
            // Empty Enter or a single space → YES (matches the capital Y in the prompt)
            "" | " " => return true,
            s if s.eq_ignore_ascii_case("y") => return true,
            s if s.eq_ignore_ascii_case("n") => return false,
            _ => {
                if no_color() {
                    println!("hint: Please enter Y (yes) or N (no).");
                } else {
                    println!("{} Please enter Y (yes) or N (no).", "hint:".yellow());
                }
            }
        }
    }
}

/// Configuration options for the quality control pipeline.
#[derive(Debug, Default)]
pub struct QcOptions {
    /// Skip the `cargo fmt` check.
    pub skip_fmt: bool,
    /// Skip the `cargo clippy` linter.
    pub skip_clippy: bool,
    /// Skip the `cargo build` compilation.
    pub skip_build: bool,
    /// Skip the `cargo test` execution.
    pub skip_test: bool,
    /// Run in non-interactive CI mode.
    pub ci: bool,
    /// Disable colored output.
    pub no_color: bool,
}

/// Main entry point for the `cargo qc` library.
///
/// Runs the configured quality gates (fmt, clippy, build, test) sequentially,
/// writes the results to `.qc_history.md`, and saves any errors to `.qc_errors.log`.
pub fn run(options: QcOptions) -> anyhow::Result<()> {
    set_no_color(options.no_color);
    let is_tty = std::io::IsTerminal::is_terminal(&std::io::stdout());
    let disable_spinners = options.ci || !is_tty;

    if no_color() {
        log_line("Local Quality Control");
    } else {
        log_line("Local Quality Control".bold());
    }

    let current_dir = env::current_dir()?;

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

    let project_name = current_dir
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "project".to_string());

    if no_color() {
        log_line(format!("Project: {} v{}", project_name, version));
    } else {
        log_line(format!(
            "Project: {} v{}",
            project_name.bold(),
            version.cyan()
        ));
    }
    log_line(format!("Directory: {}", current_dir.display()));

    let log_dir = current_dir.join("tools").join("cargo-qc");
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir).map_err(QcError::LogDirectoryCreation)?;
    }

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

        if needs_ignore && !options.ci {
            log_warning("tools/cargo-qc/ is not in your .gitignore yet.");

            if prompt_yes_no("Automatically add tools/cargo-qc/ to .gitignore?") {
                if let Ok(mut file) = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&gitignore_path)
                {
                    let _ = writeln!(file, "\n# cargo-qc logs\ntools/cargo-qc/");
                    log_line("Added tools/cargo-qc/ to .gitignore.");
                }
            } else {
                let _ = fs::File::create(&skip_prompt_path);
                log_line("Understood — this won't be asked again.");
            }
        }
    }

    let history_file = log_dir.join(".qc_history.md");
    let errors_file = log_dir.join(".qc_errors.log");
    
    // Function to truncate terminal output while keeping logs full
    let truncate_for_term = |output: &str| -> String {
        let lines: Vec<&str> = output.lines().collect();
        if lines.len() > 15 {
            let mut truncated = lines[0..15].join("\n");
            truncated.push_str("\n... (output truncated, see logs for full details)");
            truncated
        } else {
            output.to_string()
        }
    };

    if !history_file.exists()
        && let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&history_file)
    {
        let _ = writeln!(file, "# Cargo QC History\n");
        let _ = writeln!(
            file,
            "| Date | Version | Fmt | Clippy | Build | Test | Overall |"
        );
        let _ = writeln!(file, "|---|---|---|---|---|---|---|");
    }

    let mut err_log = String::new();
    let mut fmt_pass = true;
    let mut clippy_pass = true;
    let mut build_pass = true;
    let mut test_pass = true;

    if !options.skip_fmt {
        let spinner = Spinner::start("Running cargo fmt --check", disable_spinners, options.no_color);
        let fmt_output = Command::new("cargo")
            .arg("fmt")
            .arg("--")
            .arg("--check")
            .output()
            .map_err(QcError::FmtCheck)?;
        fmt_pass = fmt_output.status.success();
        spinner.finish(fmt_pass);
        if !fmt_pass {
            err_log.push_str("--- FMT ERROR ---\n");
            err_log.push_str(&String::from_utf8_lossy(&fmt_output.stdout));
            err_log.push_str(&String::from_utf8_lossy(&fmt_output.stderr));
            err_log.push('\n');
            let term_err = truncate_for_term(&String::from_utf8_lossy(&fmt_output.stderr));
            eprintln!("{}", term_err);
            if no_color() {
                eprintln!("hint: run 'cargo fmt' to format the code.");
            } else {
                eprintln!("{}", "hint: run 'cargo fmt' to format the code.".yellow());
            }
        }
    }

    if !options.skip_clippy {
        let spinner = Spinner::start("Running cargo clippy", disable_spinners, options.no_color);
        let clippy_output = Command::new("cargo")
            .arg("clippy")
            .arg("--")
            .arg("-D")
            .arg("warnings")
            .output()
            .map_err(QcError::ClippyCheck)?;
        clippy_pass = clippy_output.status.success();
        spinner.finish(clippy_pass);
        if !clippy_pass {
            err_log.push_str("--- CLIPPY ERROR ---\n");
            err_log.push_str(&String::from_utf8_lossy(&clippy_output.stdout));
            err_log.push_str(&String::from_utf8_lossy(&clippy_output.stderr));
            err_log.push('\n');
            let term_err = truncate_for_term(&String::from_utf8_lossy(&clippy_output.stderr));
            eprintln!("{}", term_err);
            if no_color() {
                eprintln!("hint: run 'cargo clippy --fix' to automatically resolve warnings.");
            } else {
                eprintln!("{}", "hint: run 'cargo clippy --fix' to automatically resolve warnings.".yellow());
            }
        }
    }

    if !options.skip_build {
        let spinner = Spinner::start("Running cargo build", disable_spinners, options.no_color);
        let build_output = Command::new("cargo")
            .arg("build")
            .output()
            .map_err(QcError::BuildCheck)?;
        build_pass = build_output.status.success();
        spinner.finish(build_pass);
        if !build_pass {
            err_log.push_str("--- BUILD ERROR ---\n");
            err_log.push_str(&String::from_utf8_lossy(&build_output.stdout));
            err_log.push_str(&String::from_utf8_lossy(&build_output.stderr));
            err_log.push('\n');
            let term_err = truncate_for_term(&String::from_utf8_lossy(&build_output.stderr));
            eprintln!("{}", term_err);
            if no_color() {
                eprintln!("hint: fix compiler errors above.");
            } else {
                eprintln!("{}", "hint: fix compiler errors above.".yellow());
            }
        }
    }

    if !options.skip_test {
        let spinner = Spinner::start("Running cargo test", disable_spinners, options.no_color);
        let test_output = Command::new("cargo")
            .arg("test")
            .output()
            .map_err(QcError::TestCheck)?;
        test_pass = test_output.status.success();
        spinner.finish(test_pass);
        if !test_pass {
            err_log.push_str("--- TEST ERROR ---\n");
            err_log.push_str(&String::from_utf8_lossy(&test_output.stdout));
            err_log.push_str(&String::from_utf8_lossy(&test_output.stderr));
            err_log.push('\n');
            let term_err = truncate_for_term(&String::from_utf8_lossy(&test_output.stderr));
            eprintln!("{}", term_err);
            if no_color() {
                eprintln!("hint: review failing tests and try again.");
            } else {
                eprintln!("{}", "hint: review failing tests and try again.".yellow());
            }
        }
    }

    let all_passed = fmt_pass && clippy_pass && build_pass && test_pass;
    let failed_count = [fmt_pass, clippy_pass, build_pass, test_pass]
        .iter()
        .filter(|passed| !**passed)
        .count();

    // Format timestamp manually to avoid pulling time's "formatting" and "parsing" features.
    let dt = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    let date = format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        dt.year(),
        u8::from(dt.month()),
        dt.day(),
        dt.hour(),
        dt.minute()
    );

    let icon = |pass: bool| if pass { "✅" } else { "❌" };

    if let Ok(mut file) = OpenOptions::new().append(true).open(&history_file) {
        let _ = writeln!(
            file,
            "| {} | `{}` | {} | {} | {} | {} | {} |",
            date,
            version,
            icon(fmt_pass),
            icon(clippy_pass),
            icon(build_pass),
            icon(test_pass),
            icon(all_passed)
        );
    }

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

        return Err(QcError::ChecksFailed {
            failed_count,
            log_path: "tools/cargo-qc/.qc_errors.log".to_string(),
        }
        .into());
    }

    if no_color() {
        log_line("All checks passed. Log written to tools/cargo-qc/.qc_history.md");
    } else {
        log_line(
            "All checks passed. Log written to tools/cargo-qc/.qc_history.md"
                .green()
                .bold(),
        );
    }

    Ok(())
}

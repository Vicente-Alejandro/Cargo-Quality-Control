#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// Configuration options and project context parsing.
pub mod config;
/// Error definitions for the cargo-qc pipeline.
pub mod error;
/// History logging and file output formatting.
pub mod history;
/// Test and build execution wrappers.
pub mod runner;
/// Terminal spinner implementation.
pub mod spinner;
/// Telemetry handling.
pub mod telemetry;

use config::{ProjectContext, QcOptions};
use error::QcError;
use owo_colors::OwoColorize;
use std::fmt::Display;
use std::time::Instant;

const PREFIX: &str = "[cargo-qc]";
const CHECK_LABEL_WIDTH: usize = 34;

static mut GLOBAL_NO_COLOR: bool = false;

/// Sets the global no-color flag for logging functions.
fn set_no_color(no_color: bool) {
    unsafe {
        GLOBAL_NO_COLOR = no_color;
    }
}

pub(crate) fn no_color() -> bool {
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

// Re-export QcOptions for public usage
pub use config::QcOptions as Options;

/// Main entry point for the `cargo qc` library.
///
/// Runs the configured quality gates (fmt, clippy, build, test) sequentially,
/// writes the results to `.qc_history.md`, and saves any errors to `.qc_errors.log`.
pub fn run(options: QcOptions) -> anyhow::Result<()> {
    let start_time = Instant::now();
    set_no_color(options.no_color);

    let is_tty = std::io::IsTerminal::is_terminal(&std::io::stdout());
    let disable_spinners = options.ci || !is_tty;

    if no_color() {
        log_line("Local Quality Control");
    } else {
        log_line("Local Quality Control".bold());
    }

    let ctx = ProjectContext::detect()?;

    if no_color() {
        log_line(format!("Project: {} v{}", ctx.project_name, ctx.version));
    } else {
        log_line(format!(
            "Project: {} v{}",
            ctx.project_name.bold(),
            ctx.version.cyan()
        ));
    }
    log_line(format!("Directory: {}", ctx.current_dir.display()));

    ctx.ensure_gitignore(options.ci)?;

    let mut err_log = String::new();

    let fmt_pass = if !options.skip_fmt {
        runner::run_fmt(&mut err_log, disable_spinners, options.no_color)
    } else {
        true
    };

    let clippy_pass = if !options.skip_clippy {
        runner::run_clippy(
            &mut err_log,
            disable_spinners,
            options.no_color,
            options.strict,
        )
    } else {
        true
    };

    let build_pass = if !options.skip_build {
        runner::run_build(&mut err_log, disable_spinners, options.no_color)
    } else {
        true
    };

    let test_pass = if !options.skip_test {
        runner::run_test(&mut err_log, disable_spinners, options.no_color)
    } else {
        true
    };

    let all_passed = fmt_pass && clippy_pass && build_pass && test_pass;
    let failed_count = [fmt_pass, clippy_pass, build_pass, test_pass]
        .iter()
        .filter(|passed| !**passed)
        .count();

    let duration = start_time.elapsed();

    history::record_history(
        &ctx.log_dir,
        &ctx.version,
        fmt_pass,
        clippy_pass,
        build_pass,
        test_pass,
        duration,
    )?;

    if let Some(telemetry_url) = options.telemetry {
        telemetry::send(
            &telemetry_url,
            &ctx.version,
            fmt_pass,
            clippy_pass,
            build_pass,
            test_pass,
            all_passed,
            duration.as_secs_f64(),
        );
    }

    if !all_passed {
        let err_log_path = history::record_errors(&ctx.log_dir, &ctx.version, &err_log)?;
        return Err(QcError::ChecksFailed {
            failed_count,
            log_path: err_log_path,
        }
        .into());
    }

    history::log_success();
    Ok(())
}

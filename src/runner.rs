use crate::{no_color, spinner::Spinner};
use owo_colors::OwoColorize;
use std::process::Command;

fn truncate_for_term(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() > 15 {
        let mut truncated = lines[0..15].join("\n");
        truncated.push_str("\n... (output truncated, see logs for full details)");
        truncated
    } else {
        output.to_string()
    }
}

/// Runs `cargo fmt --check`.
/// Logs errors and updates the error log string if it fails.
pub fn run_fmt(err_log: &mut String, disable_spinners: bool, no_color_opt: bool) -> bool {
    let spinner = Spinner::start("Running cargo fmt --check", disable_spinners, no_color_opt);
    let output = Command::new("cargo")
        .arg("fmt")
        .arg("--")
        .arg("--check")
        .output();

    let pass = match output {
        Ok(ref out) => out.status.success(),
        Err(_) => false,
    };
    spinner.finish(pass);

    if let Ok(fmt_output) = output
        && !pass
    {
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
    pass
}

/// Runs `cargo clippy`.
/// If `strict` is true, it injects SIG maintainability `-D` flags.
/// Logs errors and updates the error log string if it fails.
pub fn run_clippy(
    err_log: &mut String,
    disable_spinners: bool,
    no_color_opt: bool,
    strict: bool,
) -> bool {
    let spinner = Spinner::start("Running cargo clippy", disable_spinners, no_color_opt);
    let mut cmd = Command::new("cargo");
    cmd.arg("clippy").arg("--").arg("-D").arg("warnings");
    if strict {
        cmd.arg("-D")
            .arg("clippy::cognitive_complexity")
            .arg("-D")
            .arg("clippy::too_many_arguments")
            .arg("-D")
            .arg("clippy::type_complexity");
    }
    let output = cmd.output();

    let pass = match output {
        Ok(ref out) => out.status.success(),
        Err(_) => false,
    };
    spinner.finish(pass);

    if let Ok(clippy_output) = output
        && !pass
    {
        err_log.push_str("--- CLIPPY ERROR ---\n");
        err_log.push_str(&String::from_utf8_lossy(&clippy_output.stdout));
        err_log.push_str(&String::from_utf8_lossy(&clippy_output.stderr));
        err_log.push('\n');
        let term_err = truncate_for_term(&String::from_utf8_lossy(&clippy_output.stderr));
        eprintln!("{}", term_err);
        if no_color() {
            eprintln!("hint: run 'cargo clippy --fix' to automatically resolve warnings.");
        } else {
            eprintln!(
                "{}",
                "hint: run 'cargo clippy --fix' to automatically resolve warnings.".yellow()
            );
        }
    }
    pass
}

/// Runs `cargo build`.
/// Logs errors and updates the error log string if it fails.
pub fn run_build(err_log: &mut String, disable_spinners: bool, no_color_opt: bool) -> bool {
    let spinner = Spinner::start("Running cargo build", disable_spinners, no_color_opt);
    let output = Command::new("cargo").arg("build").output();

    let pass = match output {
        Ok(ref out) => out.status.success(),
        Err(_) => false,
    };
    spinner.finish(pass);

    if let Ok(build_output) = output
        && !pass
    {
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
    pass
}

/// Runs `cargo test`.
/// Logs errors and updates the error log string if it fails.
pub fn run_test(err_log: &mut String, disable_spinners: bool, no_color_opt: bool) -> bool {
    let spinner = Spinner::start("Running cargo test", disable_spinners, no_color_opt);
    let output = Command::new("cargo").arg("test").output();

    let pass = match output {
        Ok(ref out) => out.status.success(),
        Err(_) => false,
    };
    spinner.finish(pass);

    if let Ok(test_output) = output
        && !pass
    {
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
    pass
}

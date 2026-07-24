pub mod error;

use chrono::Local;
use colored::*;
use dialoguer::{Select, theme::ColorfulTheme};
use error::QcError;
use std::env;
use std::fmt::Display;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::process::Command;

const PREFIX: &str = "[cargo-qc]";
const CHECK_LABEL_WIDTH: usize = 34;

pub fn log_line(message: impl Display) {
    println!("{} {}", PREFIX.dimmed(), message);
}

pub fn log_check(label: &str, passed: bool) {
    let icon = if passed { "✅" } else { "❌" };
    println!("{} {label:<CHECK_LABEL_WIDTH$}{icon}", PREFIX.dimmed());
}

pub fn log_error(message: impl Display) {
    eprintln!("{} {message}", "error:".red().bold());
}

pub fn log_warning(message: impl Display) {
    println!("{} {message}", "warning:".yellow().bold());
}

pub fn run() -> anyhow::Result<()> {
    log_line("Local Quality Control".bold());

    let current_dir = env::current_dir()?;

    // Extract Version
    let mut version = String::from("unknown");
    if let Ok(output) = Command::new("cargo").arg("pkgid").output() {
        if output.status.success() {
            let pkgid = String::from_utf8_lossy(&output.stdout);
            if let Some((_, v)) = pkgid.rsplit_once('@') {
                version = v.trim().to_string();
            } else if let Some((_, v)) = pkgid.rsplit_once('#') {
                version = v.trim().to_string();
            }
        }
    }

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

    let log_dir = current_dir.join("tools").join("cargo-qc");
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir).map_err(QcError::LogDirectoryCreation)?;
    }

    let gitignore_path = current_dir.join(".gitignore");
    let skip_prompt_path = log_dir.join(".skip_gitignore_prompt");

    if !skip_prompt_path.exists() {
        let mut needs_ignore = true;
        if gitignore_path.exists() {
            if let Ok(mut file) = fs::File::open(&gitignore_path) {
                let mut content = String::new();
                let _ = file.read_to_string(&mut content);
                if content.contains("tools/cargo-qc") {
                    needs_ignore = false;
                }
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

    if !history_file.exists() {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&history_file)
        {
            let _ = writeln!(file, "# Cargo QC History\n");
            let _ = writeln!(file, "| Date | Version | Fmt | Clippy | Build | Overall |");
            let _ = writeln!(file, "|---|---|---|---|---|---|");
        }
    }

    let mut err_log = String::new();

    let fmt_output = Command::new("cargo")
        .arg("fmt")
        .arg("--")
        .arg("--check")
        .output()
        .map_err(QcError::FmtCheck)?;
    let fmt_pass = fmt_output.status.success();
    log_check("Running cargo fmt --check ...", fmt_pass);
    if !fmt_pass {
        err_log.push_str("--- FMT ERROR ---\n");
        err_log.push_str(&String::from_utf8_lossy(&fmt_output.stdout));
        err_log.push_str(&String::from_utf8_lossy(&fmt_output.stderr));
        err_log.push('\n');
    }

    let clippy_output = Command::new("cargo")
        .arg("clippy")
        .arg("--")
        .arg("-D")
        .arg("warnings")
        .output()
        .map_err(QcError::ClippyCheck)?;
    let clippy_pass = clippy_output.status.success();
    log_check("Running cargo clippy ...", clippy_pass);
    if !clippy_pass {
        err_log.push_str("--- CLIPPY ERROR ---\n");
        err_log.push_str(&String::from_utf8_lossy(&clippy_output.stdout));
        err_log.push_str(&String::from_utf8_lossy(&clippy_output.stderr));
        err_log.push('\n');
        eprint!("{}", String::from_utf8_lossy(&clippy_output.stderr));
    }

    let build_output = Command::new("cargo")
        .arg("build")
        .output()
        .map_err(QcError::BuildCheck)?;
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

    log_line(
        "All checks passed. Log written to tools/cargo-qc/.qc_history.md"
            .green()
            .bold(),
    );

    Ok(())
}

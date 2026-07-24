use chrono::Local;
use colored::*;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

fn main() {
    println!("{}", "🚀 Running Local Quality Control (cargo qc)".bold().cyan());

    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("📍 Directory: {}\n", current_dir.display());

    // Extract Version
    let mut version = String::from("Unknown");
    if let Ok(output) = Command::new("cargo").arg("pkgid").output() {
        if output.status.success() {
            let pkgid = String::from_utf8_lossy(&output.stdout);
            // Parses e.g. path+file:///...#flappy_bird@0.1.0 or ...#0.1.0
            if let Some(v) = pkgid.split('@').last() {
                version = v.trim().to_string();
            } else if let Some(v) = pkgid.split('#').last() {
                version = v.trim().to_string();
            }
        }
    }
    println!("📦 Project Version: {}", version.cyan());

    // Determine log directory (Use tools/cargo-qc if it exists, else current dir)
    let log_dir = if current_dir.join("tools").join("cargo-qc").exists() {
        current_dir.join("tools").join("cargo-qc")
    } else {
        current_dir.clone()
    };

    let history_file = log_dir.join(".qc_history.md");
    let errors_file = log_dir.join(".qc_errors.log");

    // Ensure history file has header
    if !history_file.exists() {
        if let Ok(mut file) = OpenOptions::new().create(true).write(true).open(&history_file) {
            let _ = writeln!(file, "# Cargo QC History\n");
            let _ = writeln!(file, "| Date | Version | Fmt | Clippy | Build | Overall |");
            let _ = writeln!(file, "|---|---|---|---|---|---|");
        }
    }

    let mut all_passed = true;
    let mut err_log = String::new();

    // 1. Formatting
    println!("\n{}", "➔ Running cargo fmt...".yellow());
    let fmt_output = Command::new("cargo")
        .arg("fmt")
        .arg("--")
        .arg("--check")
        .output()
        .expect("Failed to execute cargo fmt");

    let fmt_pass = fmt_output.status.success();
    if fmt_pass {
        println!("{}", "✅ Formatting passed!".green());
    } else {
        println!("{}", "❌ Formatting failed.".red().bold());
        err_log.push_str("--- FMT ERROR ---\n");
        err_log.push_str(&String::from_utf8_lossy(&fmt_output.stdout));
        err_log.push_str(&String::from_utf8_lossy(&fmt_output.stderr));
        err_log.push_str("\n");
        all_passed = false;
    }

    // 2. Clippy
    println!("\n{}", "➔ Running cargo clippy...".yellow());
    // Use inherited stdout/stderr so the user can see it in real-time,
    // but to capture it we'd have to pipe. For clippy, seeing it in real time is better,
    // but the user wanted to save the error log. So we capture it.
    let clippy_output = Command::new("cargo")
        .arg("clippy")
        .arg("--")
        .arg("-D")
        .arg("warnings")
        .output()
        .expect("Failed to execute cargo clippy");

    let clippy_pass = clippy_output.status.success();
    if clippy_pass {
        println!("{}", "✅ Clippy passed!".green());
    } else {
        println!("{}", "❌ Clippy failed.".red().bold());
        err_log.push_str("--- CLIPPY ERROR ---\n");
        err_log.push_str(&String::from_utf8_lossy(&clippy_output.stdout));
        err_log.push_str(&String::from_utf8_lossy(&clippy_output.stderr));
        err_log.push_str("\n");
        // Print it to terminal as well so they know what to fix right away
        eprint!("{}", String::from_utf8_lossy(&clippy_output.stderr));
        all_passed = false;
    }

    // 3. Build
    println!("\n{}", "➔ Running cargo build...".yellow());
    let build_output =
        Command::new("cargo").arg("build").output().expect("Failed to execute cargo build");

    let build_pass = build_output.status.success();
    if build_pass {
        println!("{}", "✅ Build passed!".green());
    } else {
        println!("{}", "❌ Build failed.".red().bold());
        err_log.push_str("--- BUILD ERROR ---\n");
        err_log.push_str(&String::from_utf8_lossy(&build_output.stdout));
        err_log.push_str(&String::from_utf8_lossy(&build_output.stderr));
        err_log.push_str("\n");
        eprint!("{}", String::from_utf8_lossy(&build_output.stderr));
        all_passed = false;
    }

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

    // Write Errors if any
    if !all_passed {
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&errors_file) {
            let _ = writeln!(file, "========================================");
            let _ = writeln!(file, "DATE: {} | VERSION: {}", date, version);
            let _ = writeln!(file, "========================================");
            let _ = writeln!(file, "{}", err_log);
        }
        println!(
            "\n{}",
            "⚠️ Some checks failed. Details saved to .qc_history.md and .qc_errors.log"
                .red()
                .bold()
        );
        std::process::exit(1);
    }

    println!("\n{}", "🎉 All checks passed successfully! Traceability updated.".bold().green());
}

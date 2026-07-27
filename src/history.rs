use crate::error::QcError;
use crate::{log_line, no_color};
use owo_colors::OwoColorize;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::Duration;
use time::OffsetDateTime;

/// Handles writing to the `.qc_history.md` file.
#[allow(clippy::collapsible_if)]
pub fn record_history(
    log_dir: &Path,
    version: &str,
    fmt_pass: bool,
    clippy_pass: bool,
    build_pass: bool,
    test_pass: bool,
    duration: Duration,
) -> Result<(), QcError> {
    let history_file = log_dir.join(".qc_history.md");

    if !history_file.exists() {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&history_file)
        {
            let _ = writeln!(file, "# Cargo QC History\n");
            let _ = writeln!(
                file,
                "| Date | Version | Fmt | Clippy | Build | Test | Overall | Duration |"
            );
            let _ = writeln!(file, "|---|---|---|---|---|---|---|---|");
        }
    }

    let all_passed = fmt_pass && clippy_pass && build_pass && test_pass;
    let icon = |pass: bool| if pass { "✅" } else { "❌" };

    // Format timestamp manually
    let dt = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    let date = format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        dt.year(),
        u8::from(dt.month()),
        dt.day(),
        dt.hour(),
        dt.minute()
    );
    let duration_str = format!("{:.1}s", duration.as_secs_f64());

    if let Ok(mut file) = OpenOptions::new().append(true).open(&history_file) {
        let _ = writeln!(
            file,
            "| {} | `{}` | {} | {} | {} | {} | {} | {} |",
            date,
            version,
            icon(fmt_pass),
            icon(clippy_pass),
            icon(build_pass),
            icon(test_pass),
            icon(all_passed),
            duration_str
        );
    }

    Ok(())
}

/// Handles writing to the `.qc_errors.log` file.
pub fn record_errors(log_dir: &Path, version: &str, err_log: &str) -> Result<String, QcError> {
    let errors_file = log_dir.join(".qc_errors.log");

    let dt = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    let date = format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        dt.year(),
        u8::from(dt.month()),
        dt.day(),
        dt.hour(),
        dt.minute()
    );

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

    Ok("tools/cargo-qc/.qc_errors.log".to_string())
}

/// Logs the final success message if all checks passed.
pub fn log_success() {
    if no_color() {
        log_line("All checks passed. Log written to tools/cargo-qc/.qc_history.md");
    } else {
        log_line(
            "All checks passed. Log written to tools/cargo-qc/.qc_history.md"
                .green()
                .bold(),
        );
    }
}

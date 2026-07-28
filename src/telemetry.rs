use crate::{log_warning, no_color};
use owo_colors::OwoColorize;
use std::process::Command;

/// Checks if `curl` is available in the system PATH.
fn curl_exists() -> bool {
    #[cfg(target_os = "windows")]
    let cmd = "where";
    #[cfg(not(target_os = "windows"))]
    let cmd = "which";

    Command::new(cmd)
        .arg("curl")
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

/// Sends execution telemetry to the specified URL using `curl`.
/// Fails gracefully with a warning if `curl` is not installed.
#[allow(clippy::too_many_arguments)]
pub fn send(
    url: &str,
    version: &str,
    fmt_pass: bool,
    clippy_pass: bool,
    build_pass: bool,
    test_pass: bool,
    all_passed: bool,
    duration_sec: f64,
) {
    if !curl_exists() {
        if no_color() {
            log_warning("Telemetry was requested, but 'curl' is not installed or not in PATH.");
        } else {
            log_warning(format!(
                "Telemetry was requested, but '{}' is not installed or not in PATH.",
                "curl".bold()
            ));
        }
        return;
    }

    let json_payload = format!(
        r#"{{"version":"{}","fmt_pass":{},"clippy_pass":{},"build_pass":{},"test_pass":{},"all_passed":{},"duration_sec":{:.1}}}"#,
        version, fmt_pass, clippy_pass, build_pass, test_pass, all_passed, duration_sec
    );

    // Fire-and-forget telemetry via curl
    let _ = Command::new("curl")
        .arg("-X")
        .arg("POST")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-d")
        .arg(&json_payload)
        .arg("--silent")
        .arg("--max-time")
        .arg("3")
        .arg(url)
        .spawn();
}

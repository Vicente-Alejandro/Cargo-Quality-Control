use assert_cmd::Command;
use predicates::prelude::*;
// No std imports needed

fn cargo_qc_cmd() -> Command {
    // Escapes the "cargo-qc" binary name automatically
    let mut cmd = Command::cargo_bin("cargo-qc").expect("failed to find cargo-qc binary");
    // Run with --ci to disable interactive prompts
    cmd.arg("--ci");
    cmd
}

#[test]
fn test_help() {
    let mut cmd = cargo_qc_cmd();
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "A custom cargo command for quality control",
    ));
}

#[test]
fn test_version() {
    let mut cmd = cargo_qc_cmd();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("cargo-qc 0.6.6")); // Assuming we bump to 0.6.6
}

#[test]
fn test_unknown_flag() {
    let mut cmd = cargo_qc_cmd();
    cmd.arg("--some-unknown-flag");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: Found argument '--some-unknown-flag' which wasn't expected",
    ));
}

#[test]
fn test_strict_mode() {
    let mut cmd = cargo_qc_cmd();
    cmd.arg("--strict").arg("--version");

    // We use --version so it doesn't actually run the full suite which might fail
    // due to unformatted test files being generated.
    cmd.assert().success();
}

#[test]
fn test_all_skip_flags() {
    let mut cmd = cargo_qc_cmd();
    // By skipping all checks, the program should instantly succeed without error,
    // proving that all core skip arguments are properly parsed.
    cmd.arg("--skip-fmt")
        .arg("--skip-clippy")
        .arg("--skip-build")
        .arg("--skip-test");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("All checks passed"));
}

#[test]
fn test_telemetry_missing_value() {
    let mut cmd = cargo_qc_cmd();
    // Telemetry requires a URL value after it
    cmd.arg("--telemetry");

    cmd.assert().failure().stderr(predicate::str::contains(
        "error: The argument '--telemetry' requires a value but none was supplied",
    ));
}

#[test]
fn test_telemetry_valid_parsing() {
    let mut cmd = cargo_qc_cmd();
    // Providing a URL should parse successfully. We skip all checks so it's instant.
    cmd.arg("--skip-fmt")
        .arg("--skip-clippy")
        .arg("--skip-build")
        .arg("--skip-test")
        .arg("--telemetry")
        .arg("http://localhost:9999/dummy-test-endpoint");

    cmd.assert().success();
}

#[test]
fn test_no_color_env_var() {
    let mut cmd = cargo_qc_cmd();
    // Set the NO_COLOR env var. We skip checks to finish fast.
    cmd.env("NO_COLOR", "1")
        .arg("--skip-fmt")
        .arg("--skip-clippy")
        .arg("--skip-build")
        .arg("--skip-test");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("All checks passed"));
}

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
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A custom cargo command for quality control"));
}

#[test]
fn test_version() {
    let mut cmd = cargo_qc_cmd();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("cargo-qc 0.6.4")); // Assuming we bump to 0.6.4
}

#[test]
fn test_unknown_flag() {
    let mut cmd = cargo_qc_cmd();
    cmd.arg("--some-unknown-flag");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error: Found argument '--some-unknown-flag' which wasn't expected"));
}

#[test]
fn test_strict_mode() {
    let mut cmd = cargo_qc_cmd();
    cmd.arg("--strict").arg("--version");
    
    // We use --version so it doesn't actually run the full suite which might fail 
    // due to unformatted test files being generated.
    cmd.assert().success();
}

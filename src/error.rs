use thiserror::Error;

/// The error type for `cargo-qc` operations.
/// Represents all possible failure modes when executing the quality control pipeline.
#[derive(Error, Debug)]
pub enum QcError {
    /// Failed to execute the `cargo pkgid` command.
    #[error("could not detect project version: {0}")]
    VersionDetection(#[from] std::io::Error),

    /// The output of `cargo pkgid` was not in the expected format.
    #[error("failed to parse cargo pkgid output")]
    VersionParse,

    /// Failed to create the `tools/cargo-qc` directory.
    #[error("failed to create log directory: {0}")]
    LogDirectoryCreation(std::io::Error),

    /// The `cargo fmt` check failed to execute.
    #[error("failed to run cargo fmt: {0}")]
    FmtCheck(std::io::Error),

    /// The `cargo clippy` check failed to execute.
    #[error("failed to run cargo clippy: {0}")]
    ClippyCheck(std::io::Error),

    /// The `cargo build` command failed to execute.
    #[error("failed to run cargo build: {0}")]
    BuildCheck(std::io::Error),

    /// The `cargo test` command failed to execute.
    #[error("failed to run cargo test: {0}")]
    TestCheck(std::io::Error),

    /// Failed to write to `.qc_history.md`.
    #[error("failed to open history file: {0}")]
    HistoryFile(std::io::Error),

    /// Failed to write to `.qc_errors.log`.
    #[error("failed to open errors file: {0}")]
    ErrorsFile(std::io::Error),

    /// One or more quality control checks failed. This is the normal failure state when code is incorrect.
    #[error("{failed_count} check(s) failed. See {log_path} for details.")]
    ChecksFailed {
        /// Number of checks that failed.
        failed_count: usize,
        /// Path to the error log file.
        log_path: String,
    },
}

use std::fmt;

/// The error type for `cargo-qc` operations.
/// Represents all possible failure modes when executing the quality control pipeline.
#[derive(Debug)]
pub enum QcError {
    /// Failed to execute the `cargo pkgid` command.
    VersionDetection(std::io::Error),

    /// The output of `cargo pkgid` was not in the expected format.
    VersionParse,

    /// Failed to create the `tools/cargo-qc` directory.
    LogDirectoryCreation(std::io::Error),

    /// The `cargo fmt` check failed to execute.
    FmtCheck(std::io::Error),

    /// The `cargo clippy` check failed to execute.
    ClippyCheck(std::io::Error),

    /// The `cargo build` command failed to execute.
    BuildCheck(std::io::Error),

    /// The `cargo test` command failed to execute.
    TestCheck(std::io::Error),

    /// Failed to write to `.qc_history.md`.
    HistoryFile(std::io::Error),

    /// Failed to write to `.qc_errors.log`.
    ErrorsFile(std::io::Error),

    /// One or more quality control checks failed. This is the normal failure state when code is incorrect.
    ChecksFailed {
        /// Number of checks that failed.
        failed_count: usize,
        /// Path to the error log file.
        log_path: String,
    },
}

impl fmt::Display for QcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QcError::VersionDetection(e) => write!(f, "could not detect project version: {}", e),
            QcError::VersionParse => write!(f, "failed to parse cargo pkgid output"),
            QcError::LogDirectoryCreation(e) => write!(f, "failed to create log directory: {}", e),
            QcError::FmtCheck(e) => write!(f, "failed to run cargo fmt: {}", e),
            QcError::ClippyCheck(e) => write!(f, "failed to run cargo clippy: {}", e),
            QcError::BuildCheck(e) => write!(f, "failed to run cargo build: {}", e),
            QcError::TestCheck(e) => write!(f, "failed to run cargo test: {}", e),
            QcError::HistoryFile(e) => write!(f, "failed to open history file: {}", e),
            QcError::ErrorsFile(e) => write!(f, "failed to open errors file: {}", e),
            QcError::ChecksFailed {
                failed_count,
                log_path,
            } => {
                write!(
                    f,
                    "{} check(s) failed. See {} for details.",
                    failed_count, log_path
                )
            }
        }
    }
}

impl std::error::Error for QcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            QcError::VersionDetection(e)
            | QcError::LogDirectoryCreation(e)
            | QcError::FmtCheck(e)
            | QcError::ClippyCheck(e)
            | QcError::BuildCheck(e)
            | QcError::TestCheck(e)
            | QcError::HistoryFile(e)
            | QcError::ErrorsFile(e) => Some(e),
            QcError::VersionParse | QcError::ChecksFailed { .. } => None,
        }
    }
}

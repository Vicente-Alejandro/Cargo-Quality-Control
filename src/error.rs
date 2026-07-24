use thiserror::Error;

#[derive(Error, Debug)]
pub enum QcError {
    #[error("could not detect project version: {0}")]
    VersionDetection(#[from] std::io::Error),

    #[error("failed to parse cargo pkgid output")]
    VersionParse,

    #[error("failed to create log directory: {0}")]
    LogDirectoryCreation(std::io::Error),

    #[error("failed to run cargo fmt: {0}")]
    FmtCheck(std::io::Error),

    #[error("failed to run cargo clippy: {0}")]
    ClippyCheck(std::io::Error),

    #[error("failed to run cargo build: {0}")]
    BuildCheck(std::io::Error),

    #[error("failed to run cargo test: {0}")]
    TestCheck(std::io::Error),

    #[error("failed to open history file: {0}")]
    HistoryFile(std::io::Error),

    #[error("failed to open errors file: {0}")]
    ErrorsFile(std::io::Error),

    #[error("{failed_count} check(s) failed. See {log_path} for details.")]
    ChecksFailed {
        failed_count: usize,
        log_path: String,
    },
}

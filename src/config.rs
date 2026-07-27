use crate::error::QcError;
use crate::{PREFIX, log_line, log_warning, no_color};
use owo_colors::OwoColorize;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Read, Write};
use std::path::PathBuf;
use std::process::Command;

/// Configuration options for the quality control pipeline.
#[derive(Debug, Default, Clone)]
pub struct QcOptions {
    /// Skip the `cargo fmt` check.
    pub skip_fmt: bool,
    /// Skip the `cargo clippy` linter.
    pub skip_clippy: bool,
    /// Skip the `cargo build` compilation.
    pub skip_build: bool,
    /// Skip the `cargo test` execution.
    pub skip_test: bool,
    /// Run in non-interactive CI mode.
    pub ci: bool,
    /// Disable colored output.
    pub no_color: bool,
    /// Enable strict maintainability lints (SIG).
    pub strict: bool,
    /// Send execution telemetry to a URL.
    pub telemetry: Option<String>,
}

/// Information about the current project context.
#[derive(Debug)]
pub struct ProjectContext {
    /// The absolute path to the current working directory.
    pub current_dir: PathBuf,
    /// The name of the project extracted from the directory name.
    pub project_name: String,
    /// The project version extracted from cargo pkgid.
    pub version: String,
    /// The path to the cargo-qc log directory.
    pub log_dir: PathBuf,
}

impl ProjectContext {
    /// Detects the current project context, including version from `cargo pkgid`.
    #[allow(clippy::collapsible_if)]
    pub fn detect() -> Result<Self, QcError> {
        let current_dir = env::current_dir().map_err(QcError::VersionDetection)?;

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

        let log_dir = current_dir.join("tools").join("cargo-qc");
        if !log_dir.exists() {
            std::fs::create_dir_all(&log_dir).map_err(QcError::LogDirectoryCreation)?;
        }

        Ok(Self {
            current_dir,
            project_name,
            version,
            log_dir,
        })
    }

    /// Verifies and prompts to ignore the `.qc_history.md` via `.gitignore`.
    #[allow(clippy::collapsible_if)]
    pub fn ensure_gitignore(&self, is_ci: bool) -> Result<(), QcError> {
        let gitignore_path = self.current_dir.join(".gitignore");
        let skip_prompt_path = self.log_dir.join(".skip_gitignore_prompt");

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

            if needs_ignore && !is_ci {
                log_warning("tools/cargo-qc/ is not in your .gitignore yet.");

                let answer = prompt_yes_no("Automatically add tools/cargo-qc/ to .gitignore?");
                match answer {
                    Ok(true) => {
                        if let Ok(mut file) = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&gitignore_path)
                        {
                            let _ = writeln!(file, "\n# cargo-qc logs\ntools/cargo-qc/");
                            log_line("Added tools/cargo-qc/ to .gitignore.");
                        }
                    }
                    Ok(false) => {
                        let _ = fs::File::create(&skip_prompt_path);
                        log_line("Understood — this won't be asked again.");
                    }
                    Err(e) => {
                        log_warning(format!(
                            "Failed to read stdin: {}. Skipping gitignore prompt.",
                            e
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}

/// Prompts the user with a `[Y/n]` question on stdout.
///
/// Accepted inputs:
/// - `y`, `Y`, a single space `' '`, or an empty Enter → returns `true` (yes)
/// - `n` or `N` → returns `false` (no)
/// - Any other input → re-prompts until a valid answer is given
///
/// Returns an error if stdin cannot be read (e.g. non-interactive without TTY).
pub fn prompt_yes_no(question: &str) -> Result<bool, io::Error> {
    let stdin = io::stdin();
    loop {
        if no_color() {
            print!("{} {} [Y/n]: ", PREFIX, question);
        } else {
            print!("{} {} [Y/n]: ", PREFIX.dimmed(), question);
        }
        io::stdout().flush()?;

        let mut line = String::new();
        let bytes_read = stdin.lock().read_line(&mut line)?;
        if bytes_read == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "EOF reached while reading stdin",
            ));
        }

        // Trim the trailing newline/carriage-return but preserve a lone space.
        let trimmed = line.trim_end_matches(['\n', '\r']);

        match trimmed {
            "" | " " => return Ok(true),
            s if s.eq_ignore_ascii_case("y") => return Ok(true),
            s if s.eq_ignore_ascii_case("n") => return Ok(false),
            _ => {
                if no_color() {
                    println!("hint: Please enter Y (yes) or N (no).");
                } else {
                    println!("{} Please enter Y (yes) or N (no).", "hint:".yellow());
                }
            }
        }
    }
}

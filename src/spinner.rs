use owo_colors::OwoColorize;
use std::io::{self, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// A simple, zero-dependency terminal spinner.
pub struct Spinner {
    stop_signal: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
    msg: String,
    disabled: bool,
    no_color: bool,
}

impl Spinner {
    /// Starts a new spinner with the given message.
    /// If `disabled` is true (e.g., in CI or non-TTY), it just prints the message once and doesn't animate.
    pub fn start(msg: impl Into<String>, disabled: bool, no_color: bool) -> Self {
        let msg = msg.into();
        let stop_signal = Arc::new(AtomicBool::new(false));
        let handle = if disabled {
            if no_color {
                print!("[cargo-qc] {} ... ", msg);
            } else {
                print!("{} {} ... ", "[cargo-qc]".dimmed(), msg);
            }
            let _ = io::stdout().flush();
            None
        } else {
            let signal = Arc::clone(&stop_signal);
            let msg_clone = msg.clone();
            Some(thread::spawn(move || {
                let mut i = 0;
                while !signal.load(Ordering::Relaxed) {
                    let frame = FRAMES[i % FRAMES.len()];
                    if no_color {
                        print!("\r[cargo-qc] {} {} ... ", frame, msg_clone);
                    } else {
                        print!(
                            "\r{} {} {} ... ",
                            "[cargo-qc]".dimmed(),
                            frame.cyan(),
                            msg_clone
                        );
                    }
                    let _ = io::stdout().flush();
                    i += 1;
                    thread::sleep(Duration::from_millis(80));
                }
            }))
        };

        Self {
            stop_signal,
            handle,
            msg,
            disabled,
            no_color,
        }
    }

    /// Stops the spinner and replaces it with a final success or failure icon.
    pub fn finish(mut self, success: bool) {
        if let Some(handle) = self.handle.take() {
            self.stop_signal.store(true, Ordering::Relaxed);
            let _ = handle.join();
        }

        let icon = if success { "✅" } else { "❌" };
        if self.disabled {
            // Already printed the prefix, just append the icon
            println!("{}", icon);
        } else {
            // Overwrite the spinner line
            let padding = 34_usize.saturating_sub(self.msg.len() + 14); // len of "Running cargo X"
            let spaces = " ".repeat(padding);
            if self.no_color {
                println!("\r[cargo-qc] {} ... {}{}", self.msg, spaces, icon);
            } else {
                println!(
                    "\r{} {} ... {}{}",
                    "[cargo-qc]".dimmed(),
                    self.msg,
                    spaces,
                    icon
                );
            }
        }
    }
}

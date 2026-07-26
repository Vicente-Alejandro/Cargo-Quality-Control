use cargo_qc::QcOptions;
use cargo_qc::log_error;

const HELP: &str = "\
cargo-qc

A custom cargo command for quality control (fmt, clippy, test etc)

USAGE:
    cargo qc [OPTIONS]

OPTIONS:
        --skip-fmt       Skip cargo fmt check
        --skip-clippy    Skip cargo clippy check
        --skip-build     Skip cargo build check
        --skip-test      Skip cargo test check
        --ci             Run in CI mode (suppress interactive prompts)
        --no-color       Disable colored output (also respects NO_COLOR env var)
    -h, --help           Print help information
    -V, --version        Print version information
";

fn main() {
    let mut options = QcOptions::default();

    // Iterator over arguments skipping the executable name.
    let mut args = std::env::args().skip(1);

    if let Some(first) = args.next() {
        // Cargo passes the subcommand name "qc" as the first argument to the plugin.
        // E.g., `cargo qc --ci` becomes `cargo-qc qc --ci`.
        let is_qc_subcommand = first == "qc";

        let mut process_arg = |arg: &str| match arg {
            "--skip-fmt" => options.skip_fmt = true,
            "--skip-clippy" => options.skip_clippy = true,
            "--skip-build" => options.skip_build = true,
            "--skip-test" => options.skip_test = true,
            "--ci" => options.ci = true,
            "--no-color" => options.no_color = true,
            "-h" | "--help" => {
                print!("{}", HELP);
                std::process::exit(0);
            }
            "-V" | "--version" => {
                println!("cargo-qc {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            _ => {
                eprintln!(
                    "error: Found argument '{}' which wasn't expected, or isn't valid in this context",
                    arg
                );
                eprintln!("\nUSAGE:\n    cargo qc [OPTIONS]");
                eprintln!("\nFor more information try --help");
                std::process::exit(1);
            }
        };

        if !is_qc_subcommand {
            process_arg(&first);
        }
        for arg in args {
            process_arg(&arg);
        }
    }

    if std::env::var("NO_COLOR").is_ok() {
        options.no_color = true;
    }

    if let Err(e) = cargo_qc::run(options) {
        log_error(e);
        std::process::exit(1);
    }
}

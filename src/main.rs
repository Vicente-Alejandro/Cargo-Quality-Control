use cargo_qc::QcOptions;
use cargo_qc::log_error;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "cargo", bin_name = "cargo")]
enum CargoCli {
    Qc(QcCli),
}

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
struct QcCli {
    /// Skip cargo fmt check
    #[arg(long)]
    skip_fmt: bool,

    /// Skip cargo clippy check
    #[arg(long)]
    skip_clippy: bool,

    /// Skip cargo build check
    #[arg(long)]
    skip_build: bool,

    /// Skip cargo test check
    #[arg(long)]
    skip_test: bool,

    /// Run in CI mode (suppress interactive prompts)
    #[arg(long)]
    ci: bool,
}

fn main() {
    let CargoCli::Qc(cli) = CargoCli::parse();

    let options = QcOptions {
        skip_fmt: cli.skip_fmt,
        skip_clippy: cli.skip_clippy,
        skip_build: cli.skip_build,
        skip_test: cli.skip_test,
        ci: cli.ci,
    };

    if let Err(e) = cargo_qc::run(options) {
        log_error(e);
        std::process::exit(1);
    }
}

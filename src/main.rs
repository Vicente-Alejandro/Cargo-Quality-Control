use cargo_qc::log_error;

fn main() {
    if let Err(e) = cargo_qc::run() {
        log_error(e);
        std::process::exit(1);
    }
}

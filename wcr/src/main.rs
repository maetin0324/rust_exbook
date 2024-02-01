fn main() {
    if let Err(e) = wcr::get_args().and_then(wcr::run) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

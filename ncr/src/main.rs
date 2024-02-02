fn main() {
    if let Err(e) = ncr::get_args().and_then(ncr::run) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

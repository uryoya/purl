fn main() {
    if let Err(e) = purl::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

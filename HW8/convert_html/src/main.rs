fn main() {
    if let Err(e) = convert::get_args().and_then(convert::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

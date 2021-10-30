fn main() {
    #[cfg(feature = "cli")]
    match netsuite::cli::run() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

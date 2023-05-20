fn main() {
    let path = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("\nError: could not find path\n\nUsage: `cargo run -- <path>.csp`\ne.g. `cargo run -- ./csp/simple.csp`");
            std::process::exit(1);
        }
    };

    apcsp::run(&path);
}

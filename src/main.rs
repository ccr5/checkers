use checkers::Checkers;
use std::process;

fn main() {
    let mut checkers: Checkers = Checkers::new();
    if let Err(e) = checkers.run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

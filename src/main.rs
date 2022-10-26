use checkers::Checkers;
use std::process;

fn main() {
    let checkers: Checkers = Checkers {};
    let (mut player_one, mut player_two, mut field) = Checkers::new();
    if let Err(e) = checkers.run(&mut player_one, &mut player_two, &mut field) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

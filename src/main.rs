use checkers::Checkers;

fn main() {
    let checkers: Checkers = Checkers {};
    let (mut player_one, mut player_two, mut field) = Checkers::new();
    checkers.run(&mut player_one, &mut player_two, &mut field);
}

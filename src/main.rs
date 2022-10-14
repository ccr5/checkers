use checkers::{field::Field, player::Player, run};

fn main() {
    println!("Welcome Ferris!");
    println!("This is my first Rust code ;)\n");

    println!("Please, who will play?");
    println!("--- Player 1 ---");
    let player_one: Player = Player::new();

    println!("\nAll right. And who will be your opponent?");
    println!("--- Player 2 ---");
    let player_two: Player = Player::new();

    let field: Field = Field::new(&player_one, &player_two);

    run(player_one, player_two, field);
}

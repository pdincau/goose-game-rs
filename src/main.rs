use crate::domain::game::{Game, Player};

mod domain;

fn main() {
    let mut game = Game::new();

    match game.add(Player::new("Pietro")) {
        Ok(_) => format(game.players()),
        Err(error) => println!("{}", error),
    };

    match game.add(Player::new("Pietro")) {
        Ok(_) => format(game.players()),
        Err(error) => println!("{}", error),
    };

    if game.add(Player::new("Paolo")).is_ok() {
        format(game.players());
    }
}

fn format(players: Vec<Player>) {
    let player_names: Vec<String> = players.iter().map(|p| p.name().to_string()).collect();
    println!("Players: {}", player_names.join(", "));
}

// add integration test on main
// separate command and query? Do we need maybe a new abstraction level?

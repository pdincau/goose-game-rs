use crate::domain::game::{Game, GameError, Player};

mod domain;

fn main() {
    let mut game = Game::new();

    match game.add(Player::new("Pietro")) {
        Ok(_) => format(game.players()),
        Err(_) => println!("Pietro: already existing player")
    };
    match game.add(Player::new("Pietro")) {
        Ok(_) => format(game.players()),
        Err(_) => println!("Pietro: already existing player")
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
// is there a way to handle errors in a more elegant way
// separate command and query? Do we need a new abstraction level?

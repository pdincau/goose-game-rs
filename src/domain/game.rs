use crate::domain::game::GameError::AlreadyInGame;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Game {
    players: Vec<Player>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Vec::new(),
        }
    }
    pub fn add(&mut self, player: Player) -> Result<(), GameError> {
        match self.find_player(player) {
            None => {
                self.players.push(player);
                Ok(())
            }
            Some(_) => Err(AlreadyInGame(player.name().to_string()))
        }
    }

    pub fn players(&self) -> Vec<Player> {
        self.players.clone()
    }

    fn find_player(&self, player: Player) -> Option<&Player> {
        self.players.iter().find(|&&p| p == player)
    }
}

pub enum GameError {
    AlreadyInGame(String),
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            AlreadyInGame(name) => write!(f, "{}: already existing player", name)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Player {
    name: &'static str,
}

impl Player {
    pub fn new(name: &'static str) -> Player {
        Player { name }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_a_player() {
        let mut game = Game::new();

        let player = Player { name: "Piero" };

        let result = game.add(player);

        assert!(result.is_ok());

        let players: Vec<Player> = game.players();

        assert_eq!(1, players.len());

        let expected_player = Player { name: "Piero" };

        match players.first() {
            None => panic!("failed!"),
            Some(actual_player) => {
                assert_eq!(expected_player, actual_player.clone())
            }
        }
    }

    #[test]
    fn cannot_add_twice_same_player() {
        let mut game = Game::new();

        let _ = game.add(Player { name: "Piero" });

        let result = game.add(Player { name: "Piero" });

        assert!(result.is_err());
    }
}

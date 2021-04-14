use std::fmt;

use thiserror::Error;

use crate::domain::game::GameError::AlreadyInGame;

pub struct Game {
    players: Players,
}

pub struct Players {
    elements: Vec<Player>,
}

impl Players {
    pub fn find(&self, player: Player) -> Option<&Player> {
        self.elements.iter().find(|&&p| p == player)
    }

    pub fn add(&mut self, player: Player) {
        self.elements.push(player);
    }

    #[allow(dead_code)]
    fn first(&self) -> Option<&Player> {
        self.elements.first()
    }

    #[allow(dead_code)]
    fn count(&self) -> usize {
        self.elements.len()
    }
}

impl Default for Players {
    fn default() -> Self {
        Players {
            elements: Vec::new(),
        }
    }
}

impl fmt::Debug for Players {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let player_names: Vec<String> = self.elements.iter().map(|p| p.name().to_string()).collect();
        write!(f, "Players: {}", player_names.join(", "))
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Players::default(),
        }
    }

    pub fn add(&mut self, player: Player) -> Result<(), GameError> {
        match self.players.find(player) {
            None => {
                self.players.add(player);
                Ok(())
            }
            Some(_) => Err(AlreadyInGame(player.name().to_string()))
        }
    }

    pub fn players(&self) -> &Players {
        &self.players
    }
}

#[derive(Debug, Error)]
pub enum GameError {
    #[error("{}: already existing player", .0)]
    AlreadyInGame(String),
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

        let players: &Players = game.players();

        assert_eq!(1, players.count());

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

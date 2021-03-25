struct Game {
    players: Vec<Player>
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Vec::new()
        }
    }
    pub fn add(&mut self, player: Player) -> Result<(), GameError> {
        self.players.push(Player { name: "Piero" });
        Ok(())
    }

    pub fn players(&self) -> Vec<Player> {
        let mut vec = Vec::new();
        vec.push(Player { name: "Piero" });
        vec
    }
}

pub enum GameError {
    GenericError
}


struct Player {
    name: &'static str
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

        assert_eq!(1, players.len())
    }
}

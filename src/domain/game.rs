struct Game {}

impl Game {
    pub fn add(self, player: Player) -> Result<(), GameError> {
        Ok(())
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
        let game = Game {};

        let player = Player { name: "Piero" };

        let result = game.add(player);

        assert!(result.is_ok());
    }
}

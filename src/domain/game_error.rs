use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("{}: already existing player", .0)]
    AlreadyInGame(String),
}

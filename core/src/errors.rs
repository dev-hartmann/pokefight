use thiserror::Error;

#[derive(Error, Debug)]
pub enum PokeFightError {
    #[error("Failed to fetch Pokemon data: {0}")]
    PokeApiError(#[from] reqwest::Error),

    #[error("Pokemon not found with ID: {0}")]
    PokemonNotFound(u32),

    #[error("Invalid Pokemon ID: {0}")]
    InvalidPokemonId(u32),

    #[error("Tournament error: {0}")]
    TournamentError(String),

    #[error("No participants provided for tournament")]
    NoParticipants,

    #[error("Invalid number of participants: {0}. Must be at least 2.")]
    InvalidParticipantCount(usize),

    #[error("Battle error: {0}")]
    BattleError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, PokeFightError>;

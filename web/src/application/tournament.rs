use pokefight_core::{PokeService, Result, Trainer, Tournament, TournamentResult};
use std::sync::Arc;

pub struct TournamentService;

impl TournamentService {
    pub async fn run_tournament(
        names: Vec<String>,
        title: String,
        chore: String,
        poke_service: Arc<PokeService>,
    ) -> Result<TournamentResult> {
        // Fetch random Pokemon for each participant
        let mut trainers = Vec::new();

        for name in names {
            let pokemon = poke_service.get_random_pokemon().await?;
            trainers.push(Trainer::new(name, pokemon));
        }

        // Create and start tournament
        let tournament = Tournament::new(trainers, &title, &chore);
        tournament.start()
    }
}

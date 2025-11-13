use clap::Parser;
use cli::{Cli, Commands, OutputFormat};
use display::TournamentDisplay;
use pokefight_core::{PokeFightError, PokeService, Result, Tournament, Trainer};

mod cli;
mod display;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let mut poke_service = PokeService::default();

    match &args.commands {
        Some(Commands::Tournament {
            names,
            title,
            chore,
            format,
        }) => {
            let mut participants: Vec<Trainer> = vec![];
            for name in names {
                let pokemon = poke_service.get_random_pokemon().await?;
                let trainer = Trainer::new(name.to_string(), pokemon);
                participants.push(trainer);
            }
            let tournament = Tournament::new(participants, title, chore);
            let tournament_result = tournament.start()?;

            // Output based on format
            match format {
                OutputFormat::Text => {
                    print!("{}", TournamentDisplay::new(&tournament_result));
                }
                OutputFormat::Json => {
                    let json = serde_json::to_string_pretty(&tournament_result).map_err(|e| {
                        PokeFightError::TournamentError(format!("JSON serialization failed: {}", e))
                    })?;
                    println!("{}", json);
                }
            }

            Ok(())
        }
        None => Err(PokeFightError::NoParticipants),
    }
}

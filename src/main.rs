use clap::Parser;
use cli::{Cli, Commands};
use models::{tournament::Tournament, trainer::Trainer};

mod cli;
mod errors;
mod models;
mod pokeservice;

use errors::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut poke_service = pokeservice::PokeService::new();

    match &cli.commands {
        Some(Commands::Tournament {
            names,
            title,
            chore,
        }) => {
            let mut participants: Vec<Trainer> = vec![];
            for name in names {
                let pokemon = poke_service.get_random_pokemon().await?;
                let trainer = Trainer::new(name.into(), pokemon);
                participants.push(trainer);
            }
            let tournament = Tournament::new(participants, title, chore);
            tournament.start();
        }
        None => println!("Need participants for tournament!"),
    }

    Ok(())
}

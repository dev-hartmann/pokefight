use clap::Parser;
use cli::{Cli, Commands};
use models::{tournament::Tournament, trainer::Trainer};

mod cli;
mod models;
mod pokeservice;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let cli = Cli::parse();
    let mut poke_service = pokeservice::PokeService::new();

    match &cli.commands {
        Some(Commands::Tournament { names, title }) => {
            let mut participants: Vec<Trainer> = vec![];
            for name in names {
                let pokemon = poke_service.get_random_pokemon().await?;
                let trainer = Trainer::new(name.into(), pokemon);
                participants.push(trainer);
            }
            let tournament = Tournament::new(participants, title);
            let winner = tournament.start();
            println!(
                "Winner of pokefight and inheritor of chore {} is:\n {}",
                tournament.get_name(),
                winner.unwrap().get_name()
            );
        }
        None => println!("Need participants for tournament!"),
    }

    Ok(())
}

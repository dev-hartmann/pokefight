use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Tournament {
        #[arg(short, long, num_args = 0..)]
        names: Vec<String>,

        #[arg(short, long)]
        title: String,

        #[arg(short, long)]
        chore: String,

        #[arg(short, long, default_value = "text")]
        format: OutputFormat,
    },
}

#[derive(Clone, clap::ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

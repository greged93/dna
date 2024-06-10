mod common;
mod ingestion;
mod inspect;

use apibara_dna_common::error::Result;
use clap::{Parser, Subcommand};
use ingestion::{run_ingestion, StartIngestionArgs};
use inspect::{run_inspect, InspectArgs};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    subcommand: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    StartIngestion(StartIngestionArgs),
    StartServer,
    Inspect(InspectArgs),
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        match self.subcommand {
            Command::StartIngestion(args) => run_ingestion(args).await,
            Command::StartServer => todo!(),
            Command::Inspect(args) => run_inspect(args).await,
        }
    }
}

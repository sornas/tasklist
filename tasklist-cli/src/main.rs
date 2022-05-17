use clap::Parser;
use color_eyre::eyre::{anyhow, Result};
use tasklists::model::Repetition;
use tracing::{event, Level};

mod http;
mod local;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(long)]
    pub local: bool,
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    #[clap(subcommand)]
    Create(Create),
    Init(Init),
    #[clap(subcommand)]
    Show(Show),
}

#[derive(clap::Subcommand, Debug)]
pub enum Create {
    Task {
        name: String,
        #[clap(long)]
        routine: usize,
    },
    Routine {
        name: String,
        #[clap(long)]
        repetition: Option<String>,
    },
}

#[derive(clap::Args, Debug)]
pub struct Init {
    #[clap(long)]
    pub routine: usize,
}

#[derive(clap::Subcommand, Debug)]
pub enum Show {
    Tasklist {
        id: u64,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .compact()
        .init();

    event!(
        Level::DEBUG,
        "parsing args from {:?}",
        std::env::args().collect::<Vec<_>>()
    );
    let args = Args::parse();
    event!(Level::DEBUG, "parsed args {:?}", args);

    if args.local {
        local::handle_args(&args)
    } else {
        http::handle_args(&args).await
    }
}

pub fn parse_repetition(s: &str) -> Result<Repetition> {
    if s == "manual" {
        Ok(Repetition::Manual)
    } else {
        Err(anyhow!("unknown repetition {:?}", s))
    }
}

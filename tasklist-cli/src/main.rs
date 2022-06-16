use clap::Parser;
use color_eyre::eyre::{anyhow, Result};
use tasklist_lib::model::{Repetition, State};
use tracing::{event, Level};
use tracing_subscriber::filter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;
use tracing_tree::HierarchicalLayer;

mod http;
// mod local;

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
    Mark(Mark),
    #[clap(subcommand)]
    Remove(Remove),
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

#[derive(clap::Parser, Debug)]
pub struct Mark {
    #[clap(long)]
    task: Option<u64>,
    #[clap(long)]
    tasklist: Option<u64>,
    state: State,
}

#[derive(clap::Subcommand, Debug)]
pub enum Remove {
    Task {
        id: u64,
        #[clap(long)]
        from: u64, // tasklist
    },
}

#[derive(clap::Subcommand, Debug)]
pub enum Show {
    Task {
        id: u64,
    },
    Tasklist {
        id: Option<u64>,
        #[clap(long)]
        follow_tasks: bool,
    },
    Routine {
        id: Option<u64>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(
        Registry::default().with(
            HierarchicalLayer::new(2)
                .with_filter(filter::Targets::new().with_target("tasklist_cli", Level::DEBUG)),
        ),
    )
    .unwrap();

    event!(
        Level::DEBUG,
        "parsing args from {:?}",
        std::env::args().collect::<Vec<_>>()
    );
    let args = Args::parse();
    event!(Level::DEBUG, ?args);

    if args.local {
        todo!()
        // local::handle_args(&args)
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

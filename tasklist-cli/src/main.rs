use clap::Parser;
use color_eyre::eyre::{anyhow, Result};
use tasklists::model::{Repetition, Routine, State, Task, TaskList};
use tracing::{event, Level};

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    #[clap(subcommand)]
    Create(Create),
    Init(Init),
}

#[derive(clap::Subcommand, Debug)]
enum Create {
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
struct Init {
    #[clap(long)]
    routine: usize,
}

fn main() -> Result<()> {
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

    handle_args(&args)?;

    Ok(())
}

fn handle_args(args: &Args) -> Result<()> {
    match &args.command {
        Command::Create(Create::Routine { name, repetition }) => {
            let mut routines = tasklists::open()?;
            let repetition = repetition
                .as_deref()
                .map(parse_repetition)
                // NOTE: legit use of transpose
                .transpose()?
                .unwrap_or(Repetition::Manual);
            routines.push(Routine {
                name: name.to_string(),
                repetition,
                model: TaskList {
                    state: State::NotStarted,
                    tasks: vec![],
                },
                task_lists: vec![],
            });
            tasklists::store(routines)?;

            Ok(())
        }

        Command::Create(Create::Task { name, routine }) => {
            let mut routines = tasklists::open()?;
            routines
                .get_mut(*routine)
                .ok_or_else(|| anyhow!("unknown routine with id {routine}"))?
                .model
                .tasks
                .push(Task {
                    state: State::NotStarted,
                    name: name.to_string(),
                });
            tasklists::store(routines)?;

            Ok(())
        }

        Command::Init(Init { routine }) => {
            let mut routines = tasklists::open()?;
            let routine = routines
                .get_mut(*routine)
                .ok_or_else(|| anyhow!("unknown routine with id {routine}"))?;
            let mut model = routine.model.clone();
            // manually started so mark as started. (a repetition trigger wouldn't mark as started.)
            model.state = State::Started;
            routine.task_lists.push(model);
            tasklists::store(routines)?;

            Ok(())
        }
    }
}

fn parse_repetition(s: &str) -> Result<Repetition> {
    if s == "manual" {
        Ok(Repetition::Manual)
    } else {
        Err(anyhow!("unknown repetition {:?}", s))
    }
}

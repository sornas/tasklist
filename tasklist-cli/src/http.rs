use color_eyre::eyre::Result;
use tasklists::model::{Repetition, Routine, State, Task, TaskList};

use crate::{parse_repetition, Args, Command, Create, Init, Show};

pub async fn handle_args(args: &Args) -> Result<()> {
    match &args.command {
        Command::Create(Create::Routine { name, repetition }) => {
            let model = TaskList {
                state: State::NotStarted,
                tasks: vec![],
            };

            let model_id = reqwest::Client::new()
                .post("http://localhost:8080/tasklist/new")
                .json(&model)
                .send()
                .await?
                .text()
                .await?
                .parse()?;

            let repetition = repetition
                .as_deref()
                .map(parse_repetition)
                .transpose()?
                .unwrap_or(Repetition::Manual);

            let routine = Routine {
                name: name.to_string(),
                repetition,
                model: model_id,
                task_lists: vec![],
            };

            let _routine = reqwest::Client::new()
                .post("http://localhost:8080/routine/new")
                .json(&routine)
                .send()
                .await?;

            Ok(())
        }

        Command::Create(Create::Task { name, routine }) => {
            let task = Task {
                state: State::NotStarted,
                name: name.to_string(),
            };

            let _task = reqwest::Client::new()
                .post(format!("http://localhost:8080/routine/{routine}/task",))
                .json(&task)
                .send()
                .await?;

            Ok(())
        }

        Command::Init(Init { routine }) => {
            let _tasklist = reqwest::Client::new()
                .post(format!("http://localhost:8080/routine/{routine}/init"))
                .send()
                .await?;

            Ok(())
        }

        Command::Show(Show::TaskList { id }) => {
            let tasklist = reqwest::Client::new()
                .get(format!("http://localhost:8080/tasklist/{id}"))
                .send()
                .await?
                .json::<TaskList>()
                .await?;
            println!("{:#?}", tasklist);
            Ok(())
        }
    }
}

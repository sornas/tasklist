use color_eyre::eyre::{anyhow, Result};
use tasklists::command;
use tasklists::model::{Repetition, Routine, State, Task, Tasklist};

use crate::{parse_repetition, Args, Command, Create, Init, Mark, Show};

pub async fn handle_args(args: &Args) -> Result<()> {
    match &args.command {
        Command::Create(Create::Routine { name, repetition }) => {
            let model = Tasklist {
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

        Command::Mark(Mark {
            task,
            tasklist,
            state,
        }) => {
            let ids_passed = [task, tasklist]
                .iter()
                .filter(|thing| thing.is_some())
                .count();
            if ids_passed == 0 {
                return Err(anyhow!("one of --task and --tasklist need to be passed"));
            } else if ids_passed > 1 {
                return Err(anyhow!("only one of --task and --tasklist may be passed"));
            }

            if let Some(task_id) = task {
                let command = command::MarkTask {
                    state: Some(state.clone()),
                    ..Default::default()
                };

                reqwest::Client::new()
                    .patch(format!("http://localhost:8080/task/{task_id}"))
                    .json(&command)
                    .send()
                    .await?;
            } else if let Some(tasklist_id) = tasklist {
                let command = command::MarkTasklist {
                    state: Some(state.clone()),
                    ..Default::default()
                };

                reqwest::Client::new()
                    .patch(format!("http://localhost:8080/tasklist/{tasklist_id}"))
                    .json(&command)
                    .send()
                    .await?;
            } else {
                unreachable!();
            }

            Ok(())
        }

        Command::Show(Show::Tasklist { id }) => {
            let tasklist = reqwest::Client::new()
                .get(format!("http://localhost:8080/tasklist/{id}"))
                .send()
                .await?
                .json::<Tasklist>()
                .await?;
            println!("{:#?}", tasklist);
            Ok(())
        }
    }
}

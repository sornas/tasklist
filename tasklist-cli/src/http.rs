use color_eyre::eyre::{anyhow, Error, Result};
use tasklists::command;
use tasklists::model::{Repetition, Routine, State, Task, Tasklist};

use crate::{parse_repetition, Args, Command, Create, Init, Mark, Remove, Show};

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

        Command::Remove(Remove::Task { id, from }) => {
            reqwest::Client::new()
                .delete(format!("http://localhost:8080/tasklist/{from}/task"))
                .json(&id)
                .send()
                .await?;
            Ok(())
        }

        Command::Show(Show::Routine { id: Some(id) }) => {
            let routine = reqwest::Client::new()
                .get(format!("http://localhost:8080/routine/{id}"))
                .send()
                .await?
                .json::<Routine>()
                .await?;
            println!("{:#?}", routine);
            Ok(())
        }

        Command::Show(Show::Routine { id: None }) => {
            let routines = reqwest::Client::new()
                .get(format!("http://localhost:8080/routine"))
                .send()
                .await?
                .json::<Vec<Routine>>()
                .await?;
            println!("{:#?}", routines);
            Ok(())
        }

        Command::Show(Show::Task { id }) => {
            let task = reqwest::Client::new()
                .get(format!("http://localhost:8080/task/{id}"))
                .send()
                .await?
                .json::<Task>()
                .await?;
            println!("{:#?}", task);
            Ok(())
        }

        Command::Show(Show::Tasklist {
            id: Some(id),
            follow_tasks,
        }) => {
            let tasklist = reqwest::Client::new()
                .get(format!("http://localhost:8080/tasklist/{id}"))
                .send()
                .await?
                .json::<Tasklist>()
                .await?;
            if *follow_tasks {
                let Tasklist {
                    state,
                    tasks: tasklist_tasks,
                } = tasklist;
                let client = reqwest::Client::new();
                // https://github.com/rust-lang/rust/issues/62290
                let mut tasks = Vec::new();
                for id in tasklist_tasks.iter() {
                    tasks.push(
                        async {
                            Result::<_, Error>::Ok(
                                client
                                    .get(format!("http://localhost:8080/task/{id}"))
                                    .send()
                                    .await?
                                    .json::<Task>()
                                    .await?,
                            )
                        }
                        .await,
                    );
                }
                println!("{:?}", state);
                println!(
                    "{:#?}",
                    tasks
                        .iter()
                        .map(|task| match task {
                            Ok(task) => format!("{task:?}"),
                            Err(err) => format!("{err}"),
                        })
                        .collect::<Vec<_>>()
                );
            } else {
                println!("{:#?}", tasklist);
            }
            Ok(())
        }

        Command::Show(Show::Tasklist {
            id: None,
            follow_tasks,
        }) => {
            if *follow_tasks {
                return Err(anyhow!("not following tasks when showing all tasklists"));
            }
            let tasklists = reqwest::Client::new()
                .get(format!("http://localhost:8080/tasklist"))
                .send()
                .await?
                .json::<Vec<Tasklist>>()
                .await?;
            println!("{:#?}", tasklists);
            Ok(())
        }
    }
}

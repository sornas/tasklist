use color_eyre::eyre::{anyhow, Error, Result};
use tap::prelude::*;
use tasklist_lib::command;
use tasklist_lib::model;
use tracing::{event, Level};

use crate::{Args, Command, Create, Init, Mark, Remove, Show};

#[tracing::instrument]
pub async fn handle_args(args: &Args) -> Result<()> {
    match &args.command {
        Command::Create(Create::Routine {
            name,
            repetition: _,
        }) => {
            let routine = model::NewRoutine { name: name.clone() };

            reqwest::Client::new()
                .post("http://localhost:8080/routines/new")
                .json(&routine)
                .send()
                .await?
                .error_for_status()?;

            Ok(())
        }

        Command::Create(Create::Task {
            name: _,
            routine: _,
        }) => {
            todo!()
            // let task = Task {
            //     state: State::NotStarted,
            //     name: name.to_string(),
            // };

            // let _task = reqwest::Client::new()
            //     .post(format!("http://localhost:8080/routine/{routine}/task",))
            //     .json(&task)
            //     .send()
            //     .await?;

            // Ok(())
        }

        Command::Init(Init { routine }) => {
            reqwest::Client::new()
                .post(format!("http://localhost:8080/routines/{routine}/init"))
                .send()
                .await?
                .error_for_status()?;

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
                    .patch(format!("http://localhost:8080/tasks/{task_id}"))
                    .json(&command)
                    .send()
                    .await?
                    .error_for_status()?;
            } else if let Some(tasklist_id) = tasklist {
                let command = command::MarkTasklist {
                    state: Some(state.clone()),
                    ..Default::default()
                };

                reqwest::Client::new()
                    .patch(format!("http://localhost:8080/tasklists/{tasklist_id}"))
                    .json(&command)
                    .send()
                    .await?
                    .error_for_status()?;
            } else {
                unreachable!();
            }

            Ok(())
        }

        Command::Remove(Remove::Task { id: _, from: _ }) => {
            todo!()
            // reqwest::Client::new()
            //     .delete(format!("http://localhost:8080/tasklist/{from}/task"))
            //     .json(&id)
            //     .send()
            //     .await?;
            // Ok(())
        }

        Command::Show(Show::Routine { id: Some(id) }) => {
            let routine = reqwest::Client::new()
                .get(format!("http://localhost:8080/routines/{id}"))
                .tap(|req| event!(Level::DEBUG, ?req))
                .send()
                .await
                .tap(|resp| event!(Level::DEBUG, ?resp))?
                .error_for_status()?
                .json::<model::Routine>()
                .await
                .tap(|body| event!(Level::DEBUG, ?body))?;
            println!("{:#?}", routine);
            Ok(())
        }

        Command::Show(Show::Routine { id: None }) => {
            let routines = reqwest::Client::new()
                .get(format!("http://localhost:8080/routines"))
                .send()
                .await?
                .error_for_status()?
                .json::<Vec<model::Routine>>()
                .await?;
            println!("{:#?}", routines);
            Ok(())
        }

        Command::Show(Show::Task { id }) => {
            let task = reqwest::Client::new()
                .get(format!("http://localhost:8080/tasks/{id}"))
                .send()
                .await?
                .error_for_status()?
                .json::<model::Task>()
                .await?;
            println!("{:#?}", task);
            Ok(())
        }

        Command::Show(Show::Tasklist {
            id: Some(id),
            follow_tasks,
        }) => {
            let tasklist = reqwest::Client::new()
                .get(format!("http://localhost:8080/tasklists/{id}"))
                .send()
                .await?
                .error_for_status()?
                .json::<model::Tasklist>()
                .await?;
            if *follow_tasks {
                let model::Tasklist {
                    id,
                    name,
                    state,
                    tasks: tasklist_tasks,
                } = tasklist;
                let client = reqwest::Client::new();
                // https://github.com/rust-lang/rust/issues/62290: async closure
                let mut tasks = Vec::new();
                for id in tasklist_tasks.iter() {
                    tasks.push(
                        async {
                            Result::<_, Error>::Ok(
                                client
                                    .get(format!("http://localhost:8080/tasks/{id}"))
                                    .send()
                                    .await?
                                    .error_for_status()?
                                    .json::<model::Task>()
                                    .await?,
                            )
                        }
                        .await,
                    );
                }
                println!("[{id}] {name} ({state:?})");
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
                .get(format!("http://localhost:8080/tasklists"))
                .send()
                .await?
                .error_for_status()?
                .json::<Vec<model::Tasklist>>()
                .await?;
            println!("{:#?}", tasklists);
            Ok(())
        }
    }
}

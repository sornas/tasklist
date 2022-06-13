use std::str::FromStr;

use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{get, web, HttpResponse, Responder};
use diesel::prelude::*;
use tasklists::model;

use crate::model as db_model;
use crate::schema;
use crate::DbPool;

#[get("/{tasklist_id}")]
async fn get(
    pool: web::Data<DbPool>,
    tasklist_id: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let tasklist_id: i32 = tasklist_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let connection = pool.get().map_err(ErrorInternalServerError)?;

    let tasklists = schema::tasklists::dsl::tasklists
        .find(tasklist_id)
        .load::<db_model::RegularTasklist>(&connection)
        .map_err(ErrorInternalServerError)?;
    let tasklist = tasklists
        .get(0)
        .ok_or_else(|| ErrorNotFound(format!("Tasklist {tasklist_id} not found")))?;

    let tasks = {
        use schema::tasklist_partof::dsl;
        dsl::tasklist_partof
            .filter(dsl::tasklist.eq(tasklist.id))
            .select(dsl::task)
            .load::<i32>(&connection)
            .map_err(ErrorInternalServerError)?
    };

    Ok(HttpResponse::Ok().json(
        &tasklist
            .clone()
            .to_model(tasks)
            .map_err(ErrorInternalServerError)?,
    ))
}

#[get("")]
async fn list(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let connection = pool.get().map_err(ErrorInternalServerError)?;

    let tasklists = schema::tasklists::dsl::tasklists
        .load::<db_model::RegularTasklist>(&connection)
        .map_err(ErrorInternalServerError)?;
    let tasklists: Vec<_> = tasklists
        .iter()
        .map(
            |db_model::RegularTasklist {
                 id,
                 name,
                 state,
                 belongs_to: _,
                 archived: _,
             }| {
                // Get all tasks that are part of this tasklist
                let tasks = {
                    use schema::tasklist_partof::dsl;
                    dsl::tasklist_partof
                        .filter(dsl::tasklist.eq(id))
                        .select(dsl::task)
                        .load::<i32>(&connection)
                        .map_err(ErrorInternalServerError)?
                };
                Ok(tasklists::model::Tasklist {
                    name: name.clone(),
                    state: model::State::from_str(state).map_err(ErrorInternalServerError)?,
                    tasks,
                })
            },
        )
        .collect::<actix_web::Result<Vec<_>>>()?;
    Ok(HttpResponse::Ok().json(&tasklists))
}

// #[patch("/{tasklist_id}")]
// async fn put(
//     tasklist_id: web::Path<String>,
//     mut command: web::Json<MarkTasklist>,
// ) -> actix_web::Result<impl Responder> {
//     let tasklist_id: usize = tasklist_id.into_inner().parse().map_err(ErrorBadRequest)?;
//
//     let mut database = tasklists::open().map_err(ErrorInternalServerError)?;
//     let mut tasklist = database
//         .tasklists
//         .get_mut(tasklist_id)
//         .ok_or(ErrorNotFound(format!("Tasklist {tasklist_id} not found")))?;
//
//     if let Some(state) = command.state.take() {
//         tasklist.state = state;
//     }
//
//     tasklists::store(&database).map_err(ErrorInternalServerError)?;
//     Ok(HttpResponse::Ok())
// }

// #[delete("/{tasklist_id}/task")]
// async fn delete_task(
//     tasklist_id: web::Path<String>,
//     task_to_remove: web::Json<u64>,
// ) -> actix_web::Result<impl Responder> {
//     let tasklist_id: usize = tasklist_id.into_inner().parse().map_err(ErrorBadRequest)?;
//     let task_to_remove = task_to_remove.into_inner();
//
//     let mut database = tasklists::open().map_err(ErrorInternalServerError)?;
//     let tasklist = database
//         .tasklists
//         .get_mut(tasklist_id)
//         .ok_or(ErrorNotFound(format!("Tasklist {tasklist_id} not found")))?;
//
//     if let Some(index) = tasklist
//         .tasks
//         .iter()
//         .enumerate()
//         .filter_map(|(idx, task)| task.eq(&task_to_remove).then(|| idx))
//         .next()
//     {
//         tasklist.tasks.remove(index);
//         tasklists::store(&database).map_err(ErrorInternalServerError)?;
//         Ok(HttpResponse::Ok())
//     } else {
//         Err(ErrorNotFound(format!(
//             "Task {task_to_remove} not found in tasklist {tasklist_id}"
//         )))
//     }
// }

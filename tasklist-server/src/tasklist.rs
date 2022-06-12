use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use tasklists::command::MarkTasklist;
use tasklists::model::Tasklist;

use crate::model;
use crate::schema::tasklists::dsl;
use crate::DbPool;

#[get("/{tasklist_id}")]
async fn get(
    pool: web::Data<DbPool>,
    tasklist_id: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let tasklist_id: i32 = tasklist_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let connection = pool.get().map_err(ErrorInternalServerError)?;

    let tasklists = dsl::tasklists
        .find(tasklist_id)
        .load::<model::Tasklist>(&connection)
        .map_err(ErrorInternalServerError)?;
    let tasklist = tasklists
        .get(0)
        .ok_or_else(|| ErrorNotFound(format!("Tasklist {tasklist_id} not found")))?;

    Ok(HttpResponse::Ok().json(&tasklist))
}

#[get("")]
async fn list(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let connection = pool.get().map_err(ErrorInternalServerError)?;

    let tasklists = dsl::tasklists
        .load::<model::Tasklist>(&connection)
        .map_err(ErrorInternalServerError)?;
    let tasklists: Vec<_> = tasklists
        .iter()
        .map(
            |model::Tasklist {
                 id,
                 name,
                 state,
                 belongs_to,
             }| {
                // Get all tasks that are part of this tasklist
                tasklists::model::Tasklist {
                    name: name.clone(),
                    state: todo!(),
                    tasks: todo!(),
                }
            },
        )
        .collect();
    Ok(HttpResponse::Ok().json(&tasklists))
}

#[post("/new")]
async fn new(tasklist: web::Json<Tasklist>) -> actix_web::Result<impl Responder> {
    let mut database = tasklists::open().map_err(ErrorInternalServerError)?;
    let tasklist_id = database.tasklists.len();
    database.tasklists.push(tasklist.into_inner());
    tasklists::store(&database).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(tasklist_id.to_string()))
}

#[patch("/{tasklist_id}")]
async fn put(
    tasklist_id: web::Path<String>,
    mut command: web::Json<MarkTasklist>,
) -> actix_web::Result<impl Responder> {
    let tasklist_id: usize = tasklist_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let mut database = tasklists::open().map_err(ErrorInternalServerError)?;
    let mut tasklist = database
        .tasklists
        .get_mut(tasklist_id)
        .ok_or(ErrorNotFound(format!("Tasklist {tasklist_id} not found")))?;

    if let Some(state) = command.state.take() {
        tasklist.state = state;
    }

    tasklists::store(&database).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok())
}

#[delete("/{tasklist_id}/task")]
async fn delete_task(
    tasklist_id: web::Path<String>,
    task_to_remove: web::Json<u64>,
) -> actix_web::Result<impl Responder> {
    let tasklist_id: usize = tasklist_id.into_inner().parse().map_err(ErrorBadRequest)?;
    let task_to_remove = task_to_remove.into_inner();

    let mut database = tasklists::open().map_err(ErrorInternalServerError)?;
    let tasklist = database
        .tasklists
        .get_mut(tasklist_id)
        .ok_or(ErrorNotFound(format!("Tasklist {tasklist_id} not found")))?;

    if let Some(index) = tasklist
        .tasks
        .iter()
        .enumerate()
        .filter_map(|(idx, task)| task.eq(&task_to_remove).then(|| idx))
        .next()
    {
        tasklist.tasks.remove(index);
        tasklists::store(&database).map_err(ErrorInternalServerError)?;
        Ok(HttpResponse::Ok())
    } else {
        Err(ErrorNotFound(format!(
            "Task {task_to_remove} not found in tasklist {tasklist_id}"
        )))
    }
}

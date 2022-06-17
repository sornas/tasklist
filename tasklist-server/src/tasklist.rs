use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{get, web, HttpResponse, Responder};
use tasklist_lib::db;

use crate::DbPool;

#[get("/{tasklist_id}")]
#[tracing::instrument]
async fn get(
    pool: web::Data<DbPool>,
    tasklist_id: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let tasklist_id: i32 = tasklist_id.into_inner().parse().map_err(ErrorBadRequest)?;
    let connection = pool.0.get().map_err(ErrorInternalServerError)?;

    let tasklist = db::tasklists::tasklist_by_id(&connection, tasklist_id)?;

    Ok(HttpResponse::Ok().json(&tasklist))
}

#[get("")]
#[tracing::instrument]
async fn list(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let connection = pool.0.get().map_err(ErrorInternalServerError)?;

    let tasklists = db::tasklists::all_tasklists(&connection)?;

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

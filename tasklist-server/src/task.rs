use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{get, patch, web, HttpResponse, Responder};
use tasklists::command::MarkTask;

#[get("/{task_id}")]
async fn get(task_id: web::Path<String>) -> actix_web::Result<impl Responder> {
    let task_id: usize = task_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let database = tasklists::open().map_err(ErrorInternalServerError)?;
    let task = database
        .tasks
        .get(task_id)
        .ok_or(ErrorNotFound(format!("Task {task_id} not found")))?;

    Ok(HttpResponse::Ok().json(&task))
}

#[patch("/{task_id}")]
async fn put(
    task_id: web::Path<String>,
    mut command: web::Json<MarkTask>,
) -> actix_web::Result<impl Responder> {
    let task_id: usize = task_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let mut database = tasklists::open().map_err(ErrorInternalServerError)?;
    let mut task = database
        .tasks
        .get_mut(task_id)
        .ok_or(ErrorNotFound(format!("Task {task_id} not found")))?;

    if let Some(state) = command.state.take() {
        task.state = state;
    }
    if let Some(name) = command.name.take() {
        task.name = name;
    }

    tasklists::store(&database).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok())
}
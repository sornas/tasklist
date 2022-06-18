use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{get, patch, web, HttpResponse, Responder};
use tasklist_lib::command::MarkTask;
use tasklist_lib::db;

use crate::DbPool;

#[get("/{task_id}")]
#[tracing::instrument]
async fn get(
    pool: web::Data<DbPool>,
    task_id: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let task_id: i32 = task_id.into_inner().parse().map_err(ErrorBadRequest)?;
    let connection = pool.0.get().map_err(ErrorInternalServerError)?;
    let task = db::task::task_by_id(&connection, task_id)?;
    Ok(HttpResponse::Ok().json(&task))
}

#[get("")]
#[tracing::instrument]
async fn list(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let connection = pool.0.get().map_err(ErrorInternalServerError)?;
    let tasks = db::task::all_tasks(&connection)?;
    Ok(HttpResponse::Ok().json(&tasks))
}

#[patch("/{task_id}")]
async fn put(
    pool: web::Data<DbPool>,
    task_id: web::Path<String>,
    mut command: web::Json<MarkTask>,
) -> actix_web::Result<impl Responder> {
    let task_id: i32 = task_id.into_inner().parse().map_err(ErrorBadRequest)?;
    let connection = pool.0.get().map_err(ErrorInternalServerError)?;

    if let Some(state) = command.state.take() {
        db::task::set_task_state(&connection, task_id, &state.to_string())?;
    }
    if let Some(name) = command.name.take() {
        db::task::set_task_name(&connection, task_id, &name)?;
    }

    Ok(HttpResponse::Ok())
}

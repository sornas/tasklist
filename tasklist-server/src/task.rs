use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{get, patch, web, HttpResponse, Responder};
use diesel::prelude::*;
use tasklists::command::MarkTask;

use crate::model;
use crate::schema::tasks::dsl;
use crate::DbPool;

#[get("/{task_id}")]
async fn get(
    pool: web::Data<DbPool>,
    task_id: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let task_id: i32 = task_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let connection = pool.get().map_err(ErrorInternalServerError)?;

    let tasks = dsl::tasks
        .find(task_id)
        .load::<model::Task>(&connection)
        .map_err(ErrorInternalServerError)?;
    let task = tasks
        .get(0)
        .ok_or_else(|| ErrorNotFound(format!("Task {task_id} not found")))?;

    Ok(HttpResponse::Ok().json(task.clone().to_model().map_err(ErrorInternalServerError)?))
}

#[patch("/{task_id}")]
async fn put(
    pool: web::Data<DbPool>,
    task_id: web::Path<String>,
    mut command: web::Json<MarkTask>,
) -> actix_web::Result<impl Responder> {
    let task_id: i32 = task_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let connection = pool.get().map_err(ErrorInternalServerError)?;

    if let Some(state) = command.state.take() {
        diesel::update(dsl::tasks.find(task_id))
            .set(dsl::state.eq(state.to_string()))
            .execute(&connection)
            .map_err(|_| ErrorNotFound(format!("Task {task_id} not found")))?;
    }
    if let Some(name) = command.name.take() {
        diesel::update(dsl::tasks.find(task_id))
            .set(dsl::name.eq(name))
            .execute(&connection)
            .map_err(|_| ErrorNotFound(format!("Task {task_id} not found")))?;
    }

    Ok(HttpResponse::Ok())
}

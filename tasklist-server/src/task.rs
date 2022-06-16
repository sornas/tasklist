use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{get, patch, web, HttpResponse, Responder};
use diesel::prelude::*;
use tap::prelude::*;
use tasklist_lib::command::MarkTask;
use tracing::{event, Level};

use crate::db;
use crate::db::schema::task::dsl;
use crate::DbPool;

#[get("/{task_id}")]
#[tracing::instrument]
async fn get(
    pool: web::Data<DbPool>,
    task_id: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let task_id: i32 = task_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let connection = pool.0.get().map_err(ErrorInternalServerError)?;

    let task = dsl::task
        .find(task_id)
        .first::<db::model::Task>(&connection)
        .optional()
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound(format!("Task {task_id} not found")))?;
    event!(Level::DEBUG, ?task);

    Ok(HttpResponse::Ok().json(task.clone().to_model().map_err(ErrorInternalServerError)?))
}

#[get("")]
#[tracing::instrument]
async fn list(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let connection = pool.0.get().map_err(ErrorInternalServerError)?;

    let tasks = dsl::task
        .load::<db::model::Task>(&connection)
        .tap(|tasks| event!(Level::DEBUG, ?tasks))
        .map_err(ErrorInternalServerError)?
        .iter()
        .map(|task| task.clone().to_model().map_err(ErrorInternalServerError))
        .collect::<actix_web::Result<Vec<_>>>()?;
    Ok(HttpResponse::Ok().json(&tasks))
}

#[patch("/{task_id}")]
async fn put(
    pool: web::Data<DbPool>,
    task_id: web::Path<String>,
    mut command: web::Json<MarkTask>,
) -> actix_web::Result<impl Responder> {
    let task_id: i32 = task_id.into_inner().parse().map_err(ErrorBadRequest)?;
    // TODO verify that task id exists

    let connection = pool.0.get().map_err(ErrorInternalServerError)?;

    if let Some(state) = command.state.take() {
        diesel::update(dsl::task.find(task_id))
            .set(dsl::state.eq(state.to_string()))
            .execute(&connection)
            .map_err(|_| ErrorNotFound(format!("Task {task_id} not found")))?;
    }
    if let Some(name) = command.name.take() {
        diesel::update(dsl::task.find(task_id))
            .set(dsl::name.eq(name))
            .execute(&connection)
            .map_err(|_| ErrorNotFound(format!("Task {task_id} not found")))?;
    }

    Ok(HttpResponse::Ok())
}

use actix_web::error::{ErrorInternalServerError, ErrorNotFound, Result};
use diesel::prelude::*;
use tap::prelude::*;
use tracing::{event, Level};

use crate::db;
use crate::model;

use db::schema::task::dsl;

fn eq_or<Lhs, Rhs, E>(a: Lhs, b: Rhs, err: E) -> Result<(), E>
where
    Lhs: PartialEq<Rhs>,
{
    a.eq(&b).then(|| ()).ok_or(err)
}

#[tracing::instrument(skip(connection))]
pub fn task_by_id(connection: &SqliteConnection, id: i32) -> Result<model::Task> {
    dsl::task
        .find(id)
        .first::<db::model::Task>(connection)
        .tap(|task| event!(Level::DEBUG, ?task))
        .optional()
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound(format!("Task {id} not found")))?
        .to_model()
        .map_err(ErrorInternalServerError)
}

#[tracing::instrument(skip(connection))]
pub fn all_tasks(connection: &SqliteConnection) -> Result<Vec<model::Task>> {
    dsl::task
        .load::<db::model::Task>(connection)
        .tap(|tasks| event!(Level::DEBUG, ?tasks))
        .map_err(ErrorInternalServerError)?
        .into_iter()
        .map(|task| task.to_model().map_err(ErrorInternalServerError))
        .collect()
}

#[tracing::instrument(skip(connection))]
pub fn set_task_state(connection: &SqliteConnection, id: i32, state: &str) -> Result<()> {
    diesel::update(dsl::task.find(id))
        .set(dsl::state.eq(state.to_string()))
        .execute(connection)
        .map_err(ErrorInternalServerError)
        .and_then(|updated| eq_or(updated, 1, ErrorNotFound(format!("Task {id} not found"))))
}

#[tracing::instrument(skip(connection))]
pub fn set_task_name(connection: &SqliteConnection, id: i32, name: &str) -> Result<()> {
    diesel::update(dsl::task.find(id))
        .set(dsl::name.eq(name))
        .execute(connection)
        .map_err(ErrorInternalServerError)
        .and_then(|updated| eq_or(updated, 1, ErrorNotFound(format!("Task {id} not found"))))
}

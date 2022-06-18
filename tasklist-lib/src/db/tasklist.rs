use actix_web::error::{ErrorInternalServerError, ErrorNotFound, Result};
use diesel::prelude::*;
use tap::prelude::*;
use tracing::{event, Level};

use crate::db;
use crate::model;

use db::schema::{self, tasklist::dsl};

fn tasks_in_tasklist(connection: &SqliteConnection, tasklist: i32) -> Result<Vec<i32>> {
    use schema::task_partof_regular::dsl;
    dsl::task_partof_regular
        .filter(dsl::regular.eq(tasklist))
        .select(dsl::task)
        .load(connection)
        .map_err(ErrorInternalServerError)
}

#[tracing::instrument(skip(connection))]
pub fn all_tasklists(connection: &SqliteConnection) -> Result<Vec<model::Tasklist>> {
    dsl::tasklist
        .load::<db::model::RegularTasklist>(connection)
        .map_err(ErrorInternalServerError)?
        .iter()
        .map(|tasklist| {
            let tasks = tasks_in_tasklist(connection, tasklist.id)?;
            tasklist
                .clone()
                .to_model(tasks)
                .map_err(ErrorInternalServerError)
        })
        .collect()
}

#[tracing::instrument(skip(connection))]
pub fn tasklist_by_id(connection: &SqliteConnection, id: i32) -> Result<model::Tasklist> {
    let tasklist = dsl::tasklist
        .find(id)
        .first::<db::model::RegularTasklist>(connection)
        .tap(|tasklist| event!(Level::DEBUG, ?tasklist))
        .optional()
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound(format!("Tasklist {id} not found")))?;

    let tasks = tasks_in_tasklist(connection, tasklist.id)?;
    tasklist.to_model(tasks).map_err(ErrorInternalServerError)
}

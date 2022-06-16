use color_eyre::eyre::{anyhow, Result};
use diesel::prelude::*;
use tap::prelude::*;
use tracing::{event, Level};

use crate::db;
use crate::model;

use db::schema;

#[tracing::instrument(skip(connection))]
pub fn all_tasklists(connection: &SqliteConnection) -> Result<Vec<model::Tasklist>> {
    schema::tasklist::dsl::tasklist
        .load::<db::model::RegularTasklist>(connection)?
        .iter()
        .map(|tasklist| {
            let tasks = {
                use schema::task_partof_regular::dsl;
                dsl::task_partof_regular
                    .filter(dsl::regular.eq(tasklist.id))
                    .select(dsl::task)
                    .load::<i32>(connection)?
            };
            tasklist.clone().to_model(tasks)
        })
        .collect()
}

#[tracing::instrument(skip(connection))]
pub fn tasklist_by_id(connection: &SqliteConnection, id: i32) -> Result<model::Tasklist> {
    let tasklist = schema::tasklist::dsl::tasklist
        .find(id)
        .first::<db::model::RegularTasklist>(connection)
        .tap(|tasklist| event!(Level::DEBUG, ?tasklist))
        .optional()?
        .ok_or_else(|| anyhow!("Tasklist {id} not found"))?;

    let tasks = {
        use schema::task_partof_regular::dsl;
        dsl::task_partof_regular
            .filter(dsl::regular.eq(tasklist.id))
            .select(dsl::task)
            .load::<i32>(connection)?
    };
    tasklist.to_model(tasks)
}

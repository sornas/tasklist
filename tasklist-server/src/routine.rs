use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use tasklists::model;

use crate::db;
use crate::db::schema;
use crate::DbPool;

#[get("")]
async fn list(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let connection = pool.get().map_err(ErrorInternalServerError)?;
    let routines = schema::routine::dsl::routine
        .load::<db::model::Routine>(&connection)
        .map_err(ErrorInternalServerError)?
        .iter()
        .map(|routine| {
            let tasklists = routine
                .tasklists(&connection)
                .map_err(ErrorInternalServerError)?;
            routine
                .clone()
                .to_model(tasklists)
                .map_err(ErrorInternalServerError)
        })
        .collect::<actix_web::Result<Vec<model::Routine>>>()?;

    Ok(HttpResponse::Ok().json(&routines))
}

#[get("/{routine_id}")]
async fn get(
    pool: web::Data<DbPool>,
    routine_id: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let routine_id: i32 = routine_id.into_inner().parse().map_err(ErrorBadRequest)?;
    let connection = pool.get().map_err(ErrorInternalServerError)?;

    let routine = schema::routine::dsl::routine
        .find(routine_id)
        .first::<db::model::Routine>(&connection)
        .optional()
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound(format!("Routine {routine_id} not found")))?;

    let tasklists = routine
        .tasklists(&connection)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(
        &routine
            .clone()
            .to_model(tasklists)
            .map_err(ErrorInternalServerError)?,
    ))
}

#[post("/new")]
async fn new(
    pool: web::Data<DbPool>,
    routine: web::Json<model::NewRoutine>,
) -> actix_web::Result<impl Responder> {
    let connection = pool.get().map_err(ErrorInternalServerError)?;

    let model = db::model::insert::ModelTasklist { routine: 0 };
    let model_id = model
        .insert_and_id(&connection)
        .map_err(ErrorInternalServerError)?;

    let routine = db::model::insert::Routine {
        name: &routine.name,
        model: model_id,
    };
    let routine_id = routine
        .insert_and_id(&connection)
        .map_err(ErrorInternalServerError)?;

    {
        use schema::model::dsl;
        diesel::update(dsl::model.find(model_id))
            .set(dsl::routine.eq(routine_id))
            .execute(&connection)
            .map_err(ErrorInternalServerError)?;
    }

    Ok(HttpResponse::Ok())
}

#[post("/{routine_id}/task")]
async fn add_task(
    pool: web::Data<DbPool>,
    routine_id: web::Path<String>,
    task: web::Json<model::Task>,
) -> actix_web::Result<impl Responder> {
    let routine_id: i32 = routine_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let connection = pool.get().map_err(ErrorInternalServerError)?;

    let model_id = {
        use schema::routine::dsl;
        dsl::routine
            .find(routine_id)
            .select(dsl::model)
            .first::<i32>(&connection)
            .optional()
            .map_err(ErrorInternalServerError)?
            .ok_or_else(|| ErrorNotFound(format!("Routine {routine_id} not found")))?
    };

    let task = db::model::insert::Task::from(task.0);
    let task_id = task
        .insert_and_id(&connection)
        .map_err(ErrorInternalServerError)?;

    let partof_model = db::model::insert::TaskPartofModel {
        model: model_id,
        task: task_id,
    };
    partof_model
        .insert(&connection)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok())
}

#[get("/{routine_id}/tasks")]
async fn tasks(
    pool: web::Data<DbPool>,
    routine_id: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let routine_id: i32 = routine_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let connection = pool.get().map_err(ErrorInternalServerError)?;

    let model_id = {
        use schema::routine::dsl;
        dsl::routine
            .find(routine_id)
            .select(dsl::model)
            .first::<i32>(&connection)
            .optional()
            .map_err(ErrorInternalServerError)?
            .ok_or_else(|| ErrorNotFound(format!("Routine {routine_id} not found")))?
    };

    let tasks = {
        use schema::task_partof_model::dsl;
        dsl::task_partof_model
            .filter(dsl::model.eq(model_id))
            .select(dsl::task)
            .load::<i32>(&connection)
            .map_err(ErrorInternalServerError)?
    };

    Ok(HttpResponse::Ok().json(&tasks))
}

// #[post("/{routine_id}/init")]
// async fn init(routine_id: web::Path<String>) -> actix_web::Result<impl Responder> {
//     let routine_id: usize = routine_id.into_inner().parse().map_err(ErrorBadRequest)?;
//
//     let mut database = tasklists::open().map_err(ErrorInternalServerError)?;
//     let routine = database
//         .routines
//         .get_mut(routine_id)
//         .ok_or(ErrorNotFound(format!("Routine {routine_id} not found")))?;
//
//     let mut model = database
//         .tasklists
//         .get(routine.model as usize)
//         .ok_or(ErrorNotFound(format!(
//             "Routine {routine_id} refers to non-existant model tasklist {}",
//             routine.model
//         )))?
//         .clone();
//     model.state = State::Started;
//
//     let tasklist_id = database.tasklists.len();
//     database.tasklists.push(model);
//     routine.task_lists.push(tasklist_id as u64);
//
//     tasklists::store(&database).map_err(ErrorInternalServerError)?;
//     Ok(HttpResponse::Ok().body(tasklist_id.to_string()))
// }

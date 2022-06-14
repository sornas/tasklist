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
    let routines = schema::routines::dsl::routines
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

    let routine = schema::routines::dsl::routines
        .find(routine_id)
        .first::<db::model::Routine>(&connection)
        .map_err(|e| ErrorNotFound(format!("Routine {routine_id} not found: {e:?}")))?;

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
        use schema::models::dsl;
        diesel::update(dsl::models.find(model_id))
            .set(dsl::routine.eq(routine_id))
            .execute(&connection)
            .map_err(ErrorInternalServerError)?;
    }

    Ok(HttpResponse::Ok())
}

// #[post("/{routine_id}/task")]
// async fn add_task(
//     routine_id: web::Path<String>,
//     task: web::Json<Task>,
// ) -> actix_web::Result<impl Responder> {
//     let routine_id: usize = routine_id.into_inner().parse().map_err(ErrorBadRequest)?;
//
//     let mut database = tasklists::open().map_err(ErrorInternalServerError)?;
//
//     let task_id = database.tasks.len();
//     database.tasks.push(task.0);
//
//     let routine_model = database
//         .routines
//         .get(routine_id)
//         .ok_or(ErrorNotFound(format!("Routine {routine_id} not found")))?
//         .model;
//
//     database.tasklists[routine_model as usize]
//         .tasks
//         .push(task_id as u64);
//
//     tasklists::store(&database).map_err(ErrorInternalServerError)?;
//     Ok(HttpResponse::Ok().body(task_id.to_string()))
// }

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

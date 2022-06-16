use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use tap::prelude::*;
use tasklist_lib::db;
use tasklist_lib::db::schema;
use tasklist_lib::model;
use tracing::{event, Level};

use crate::DbPool;

#[get("")]
#[tracing::instrument]
async fn list(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let connection = pool.0.get().map_err(ErrorInternalServerError)?;
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
#[tracing::instrument]
async fn get(
    pool: web::Data<DbPool>,
    routine_id: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let routine_id: i32 = routine_id.into_inner().parse().map_err(ErrorBadRequest)?;
    let connection = pool.0.get().map_err(ErrorInternalServerError)?;

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
#[tracing::instrument]
async fn new(
    pool: web::Data<DbPool>,
    routine: web::Json<model::NewRoutine>,
) -> actix_web::Result<impl Responder> {
    let connection = pool.0.get().map_err(ErrorInternalServerError)?;

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
#[tracing::instrument]
async fn add_task(
    pool: web::Data<DbPool>,
    routine_id: web::Path<String>,
    task: web::Json<model::Task>,
) -> actix_web::Result<impl Responder> {
    let routine_id: i32 = routine_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let connection = pool.0.get().map_err(ErrorInternalServerError)?;

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
#[tracing::instrument]
async fn routine_tasks(
    pool: web::Data<DbPool>,
    routine_id: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let routine_id: i32 = routine_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let connection = pool.0.get().map_err(ErrorInternalServerError)?;

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

#[post("/{routine_id}/init")]
#[tracing::instrument]
async fn init(
    pool: web::Data<DbPool>,
    routine_id: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let routine_id: i32 = routine_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let connection = pool.0.get().map_err(ErrorInternalServerError)?;

    let (name, model_id): (String, i32) = {
        use schema::routine::dsl;

        dsl::routine
            .find(routine_id)
            .select((dsl::name, dsl::model))
            .first(&connection)
            .tap(|routine| event!(Level::TRACE, ?routine))
            .optional()
            .map_err(ErrorInternalServerError)?
            .ok_or_else(|| ErrorNotFound(format!("Routine {routine_id} not found")))?
    };

    // Insert our new tasklist
    let tasklist = db::model::insert::RegularTasklist {
        name: &name,
        state: &model::State::NotStarted.to_string(),
        routine_id,
    };
    let tasklist_id = tasklist
        .insert_and_id(&connection)
        .map_err(ErrorInternalServerError)?;

    // Get model's tasks
    let tasks: Vec<i32> = {
        use schema::task_partof_model::dsl;
        dsl::task_partof_model
            .filter(dsl::model.eq(model_id))
            .select(dsl::task)
            .load(&connection)
            .map_err(ErrorInternalServerError)?
    };
    event!(Level::TRACE, ?tasks);

    // Insert copies of tasks
    // FIXME this is one fetch for each name
    let new_tasks: Vec<i32> = tasks
        .iter()
        .map(|id| -> actix_web::Result<String> {
            use schema::task::dsl;
            let name = dsl::task
                .find(id)
                .select(dsl::name)
                .first(&connection)
                .optional();
            match name {
                Err(e) => Err(ErrorInternalServerError(e)),
                Ok(None) => Err(ErrorInternalServerError(format!(
                    "Model {model_id} in routine {routine_id} refers to invalid task {id}"
                ))),
                Ok(Some(name)) => Ok(name),
            }
        })
        .map(|name| -> actix_web::Result<_> {
            // Insert new task
            let task = db::model::insert::Task {
                name: name?,
                state: model::State::NotStarted.to_string(),
            };
            let new_id = task
                .insert_and_id(&connection)
                .map_err(ErrorInternalServerError)?;
            Ok(new_id)
        })
        .collect::<actix_web::Result<_>>()?;
    event!(Level::TRACE, ?new_tasks);

    let new_task_partof = new_tasks
        .iter()
        .map(|task| db::model::insert::TaskPartofRegular {
            regular: tasklist_id,
            task: *task,
        })
        .collect::<Vec<_>>();

    // FIXME Sqlite doesn't support inserting multiple values when using a pooled connection?
    //       so this is one insert per task.
    // diesel::insert_into(db::schema::task_partof_regular::table)
    //     .values(&new_task_partof)
    //     .execute(&connection)
    //     .map_err(ErrorInternalServerError)?;

    for to_insert in new_task_partof {
        to_insert
            .insert(&connection)
            .map_err(ErrorInternalServerError)?;
    }

    Ok(HttpResponse::Ok())
}

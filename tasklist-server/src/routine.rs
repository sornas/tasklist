use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{get, post, web, HttpResponse, Responder};
use tasklists::model::{Routine, State, Task};

#[get("/{routine_id}")]
async fn get(routine_id: web::Path<String>) -> actix_web::Result<impl Responder> {
    let routine_id: usize = routine_id.into_inner().parse().map_err(ErrorBadRequest)?;
    let database = tasklists::open().map_err(ErrorInternalServerError)?;
    let routine = database
        .routines
        .get(routine_id)
        .ok_or(ErrorNotFound(format!("Routine {routine_id} not found")))?;
    Ok(HttpResponse::Ok().json(&routine))
}

#[post("/new")]
async fn new(routine: web::Json<Routine>) -> actix_web::Result<impl Responder> {
    let mut database = tasklists::open().map_err(ErrorInternalServerError)?;
    let routine_id = database.routines.len();
    database.routines.push(routine.0);
    tasklists::store(&database).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(routine_id.to_string()))
}

#[post("/{routine_id}/task")]
async fn add_task(
    routine_id: web::Path<String>,
    task: web::Json<Task>,
) -> actix_web::Result<impl Responder> {
    let routine_id: usize = routine_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let mut database = tasklists::open().map_err(ErrorInternalServerError)?;

    let task_id = database.tasks.len();
    database.tasks.push(task.0);

    let routine_model = database
        .routines
        .get(routine_id)
        .ok_or(ErrorNotFound(format!("Routine {routine_id} not found")))?
        .model;

    database.tasklists[routine_model as usize]
        .tasks
        .push(task_id as u64);

    tasklists::store(&database).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(task_id.to_string()))
}

#[post("/{routine_id}/init")]
async fn init(routine_id: web::Path<String>) -> actix_web::Result<impl Responder> {
    let routine_id: usize = routine_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let mut database = tasklists::open().map_err(ErrorInternalServerError)?;
    let routine = database
        .routines
        .get_mut(routine_id)
        .ok_or(ErrorNotFound(format!("Routine {routine_id} not found")))?;

    let mut model = database
        .tasklists
        .get(routine.model as usize)
        .ok_or(ErrorNotFound(format!(
            "Routine {routine_id} refers to non-existant model tasklist {}",
            routine.model
        )))?
        .clone();
    model.state = State::Started;

    let tasklist_id = database.tasklists.len();
    database.tasklists.push(model);
    routine.task_lists.push(tasklist_id as u64);

    tasklists::store(&database).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(tasklist_id.to_string()))
}

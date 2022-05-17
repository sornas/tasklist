use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tasklists::model::{Routine, State, Task, TaskList};
use tracing::Level;
use tracing_actix_web::TracingLogger;

#[get("/routine/{routine_id}")]
async fn get_routine(routine_id: web::Path<String>) -> actix_web::Result<impl Responder> {
    let routine_id: usize = routine_id.into_inner().parse().map_err(ErrorBadRequest)?;
    let database = tasklists::open().map_err(ErrorInternalServerError)?;
    let routine = database
        .routines
        .get(routine_id)
        .ok_or(ErrorNotFound(format!("Routine {routine_id} not found")))?;
    Ok(HttpResponse::Ok().json(&routine))
}

#[post("/routine/new")]
async fn new_routine(routine: web::Json<Routine>) -> actix_web::Result<impl Responder> {
    let mut database = tasklists::open().map_err(ErrorInternalServerError)?;
    let routine_id = database.routines.len();
    database.routines.push(routine.0);
    tasklists::store(&database).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(routine_id.to_string()))
}

#[post("/routine/{routine_id}/task")]
async fn add_task_to_routine(
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

#[post("/routine/{routine_id}/init")]
async fn init_routine(routine_id: web::Path<String>) -> actix_web::Result<impl Responder> {
    let routine_id: usize = routine_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let mut database = tasklists::open().map_err(ErrorInternalServerError)?;

    let routine_model = database
        .routines
        .get(routine_id)
        .ok_or(ErrorNotFound(format!("Routine {routine_id} not found")))?
        .model;

    let mut model = database
        .tasklists
        .get(routine_model as usize)
        .ok_or(ErrorNotFound(format!(
            "Routine {routine_id} refers to non-existant model tasklist {routine_model}"
        )))?
        .clone();
    model.state = State::Started;

    let tasklist_id = database.tasklists.len();
    database.tasklists.push(model);

    tasklists::store(&database).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(tasklist_id.to_string()))
}

#[get("/tasklist/{tasklist_id}")]
async fn get_tasklist(tasklist_id: web::Path<String>) -> actix_web::Result<impl Responder> {
    let tasklist_id: usize = tasklist_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let database = tasklists::open().map_err(ErrorInternalServerError)?;
    let tasklist = database
        .tasklists
        .get(tasklist_id)
        .ok_or(ErrorNotFound(format!("Tasklist {tasklist_id} not found")))?;

    Ok(HttpResponse::Ok().json(&tasklist))
}

#[post("/tasklist/new")]
async fn new_tasklist(tasklist: web::Json<TaskList>) -> actix_web::Result<impl Responder> {
    let mut database = tasklists::open().map_err(ErrorInternalServerError)?;
    let tasklist_id = database.tasklists.len();
    database.tasklists.push(tasklist.into_inner());
    tasklists::store(&database).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(tasklist_id.to_string()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .compact()
        .init();

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .service(add_task_to_routine)
            .service(get_routine)
            .service(get_tasklist)
            .service(new_routine)
            .service(new_tasklist)
            .service(init_routine)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

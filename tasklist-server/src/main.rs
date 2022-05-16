use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tasklists::model::{Routine, State, Task};
use tracing::Level;
use tracing_actix_web::TracingLogger;

#[tracing::instrument]
#[get("/routine/{routine_id}")]
async fn get_routine(routine_id: web::Path<String>) -> actix_web::Result<impl Responder> {
    let routine_id: usize = routine_id.into_inner().parse().map_err(ErrorBadRequest)?;
    let database = tasklists::open().map_err(ErrorInternalServerError)?;
    let routine = database
        .routines
        .get(routine_id)
        .ok_or(ErrorNotFound(format!("Routine {routine_id} not found")))?;
    let json = serde_json::to_string(&routine).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json))
}

#[tracing::instrument]
#[post("/routine/new")]
async fn add_new_routine(routine: web::Json<Routine>) -> actix_web::Result<impl Responder> {
    let mut database = tasklists::open().map_err(ErrorInternalServerError)?;
    let routine_id = database.routines.len();
    database.routines.push(routine.0);
    tasklists::store(&database).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(routine_id.to_string()))
}

#[tracing::instrument]
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
    Ok(HttpResponse::Ok())
}

#[tracing::instrument]
#[post("/routine/{routine_id}/init")]
async fn init_routine(routine_id: web::Path<String>) -> actix_web::Result<impl Responder> {
    let routine_id: usize = routine_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let mut database = tasklists::open().map_err(ErrorInternalServerError)?;

    let routine_model = database
        .routines
        .get(routine_id)
        .ok_or(ErrorNotFound(format!("Routine {routine_id} not found")))?
        .model;

    let mut model = database.tasklists[routine_model as usize].clone();
    model.state = State::Started;
    database.tasklists.push(model);

    tasklists::store(&database).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok())
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
            .service(add_new_routine)
            .service(add_task_to_routine)
            .service(get_routine)
            .service(init_routine)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

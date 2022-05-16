use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tasklists::model::{Routine, Task};
use tracing::Level;
use tracing_actix_web::TracingLogger;

#[tracing::instrument]
#[get("/routine/{routine_id}")]
async fn get_routine(routine_id: web::Path<String>) -> actix_web::Result<impl Responder> {
    let routine_id: usize = routine_id.into_inner().parse().map_err(ErrorBadRequest)?;
    let routines = tasklists::open().map_err(ErrorInternalServerError)?;
    let routine = routines
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
    let mut routines = tasklists::open().map_err(ErrorInternalServerError)?;
    routines.push(routine.0);
    tasklists::store(routines).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok())
}

#[tracing::instrument]
#[post("/routine/{routine_id}/task")]
async fn add_task_to_routine(
    routine_id: web::Path<String>,
    task: web::Json<Task>,
) -> actix_web::Result<impl Responder> {
    let routine_id: usize = routine_id.into_inner().parse().map_err(ErrorBadRequest)?;
    let mut routines = tasklists::open().map_err(ErrorInternalServerError)?;
    let routine = routines
        .get_mut(routine_id)
        .ok_or(ErrorNotFound(format!("Routine {routine_id} not found")))?;
    routine.model.tasks.push(task.0);
    tasklists::store(routines).map_err(ErrorInternalServerError)?;
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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

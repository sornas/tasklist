use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{get, patch, post, web, HttpResponse, Responder};
use tasklists::command::MarkTasklist;
use tasklists::model::Tasklist;

#[get("/{tasklist_id}")]
async fn get(tasklist_id: web::Path<String>) -> actix_web::Result<impl Responder> {
    let tasklist_id: usize = tasklist_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let database = tasklists::open().map_err(ErrorInternalServerError)?;
    let tasklist = database
        .tasklists
        .get(tasklist_id)
        .ok_or(ErrorNotFound(format!("Tasklist {tasklist_id} not found")))?;

    Ok(HttpResponse::Ok().json(&tasklist))
}

#[get("/")]
async fn list() -> actix_web::Result<impl Responder> {
    let database = tasklists::open().map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(&database.tasklists))
}

#[post("/new")]
async fn new(tasklist: web::Json<Tasklist>) -> actix_web::Result<impl Responder> {
    let mut database = tasklists::open().map_err(ErrorInternalServerError)?;
    let tasklist_id = database.tasklists.len();
    database.tasklists.push(tasklist.into_inner());
    tasklists::store(&database).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(tasklist_id.to_string()))
}

#[patch("/{tasklist_id}")]
async fn put(
    tasklist_id: web::Path<String>,
    mut command: web::Json<MarkTasklist>,
) -> actix_web::Result<impl Responder> {
    let tasklist_id: usize = tasklist_id.into_inner().parse().map_err(ErrorBadRequest)?;

    let mut database = tasklists::open().map_err(ErrorInternalServerError)?;
    let mut tasklist = database
        .tasklists
        .get_mut(tasklist_id)
        .ok_or(ErrorNotFound(format!("Tasklist {tasklist_id} not found")))?;

    if let Some(state) = command.state.take() {
        tasklist.state = state;
    }

    tasklists::store(&database).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok())
}

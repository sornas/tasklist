use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tasklists::model::Tasklist;
use tracing::Level;
use tracing_actix_web::TracingLogger;

mod routine;

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
async fn new_tasklist(tasklist: web::Json<Tasklist>) -> actix_web::Result<impl Responder> {
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
            .service(web::scope("/routine")
                .service(routine::add_task)
                .service(routine::get)
                .service(routine::init)
                .service(routine::new)
            )
            .service(get_tasklist)
            .service(new_tasklist)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

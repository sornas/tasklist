#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2;
use r2d2::ConnectionManager;
use tracing::Level;
use tracing_actix_web::TracingLogger;

mod db;
mod routine;
mod task;
mod tasklist;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

diesel_migrations::embed_migrations!("migrations");

fn db_connection() -> Result<SqliteConnection, ConnectionError> {
    SqliteConnection::establish("tasklist.sqlite")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .compact()
        .init();

    let connection = db_connection().unwrap();
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();

    let manager = ConnectionManager::<SqliteConnection>::new("tasklist.sqlite");
    let pool: DbPool = r2d2::Pool::builder().build(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(actix_web::middleware::NormalizePath::trim())
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/routines")
                    // .service(routine::init)
                    .service(routine::add_task)
                    .service(routine::get)
                    .service(routine::list)
                    .service(routine::new)
                    .service(routine::tasks),
            )
            .service(
                web::scope("/tasks")
                    .service(task::get)
                    .service(task::list)
                    .service(task::put),
            )
            .service(
                web::scope("/tasklists")
                    // .service(tasklist::delete_task)
                    // .service(tasklist::put),
                    .service(tasklist::get)
                    .service(tasklist::list),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

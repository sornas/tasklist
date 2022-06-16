#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2;
use r2d2::ConnectionManager;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;
use tracing_tree::HierarchicalLayer;

mod db;
mod routine;
mod task;
mod tasklist;

type InnerPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Clone)]
struct DbPool(r2d2::Pool<ConnectionManager<SqliteConnection>>);

impl DbPool {
    fn build() -> Self {
        let manager = ConnectionManager::<SqliteConnection>::new("tasklist.sqlite");
        let pool = r2d2::Pool::builder().build(manager).unwrap();
        Self(pool)
    }
}

impl std::fmt::Debug for DbPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DbPool").finish_non_exhaustive()
    }
}

impl AsRef<InnerPool> for DbPool {
    fn as_ref(&self) -> &InnerPool {
        &self.0
    }
}

diesel_migrations::embed_migrations!("migrations");

fn db_connection() -> Result<SqliteConnection, ConnectionError> {
    SqliteConnection::establish("tasklist.sqlite")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = Registry::default().with(HierarchicalLayer::new(2));
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let connection = db_connection().unwrap();
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();

    let pool = DbPool::build();

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(actix_web::middleware::NormalizePath::trim())
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/routines")
                    .service(routine::add_task)
                    .service(routine::get)
                    .service(routine::init)
                    .service(routine::list)
                    .service(routine::new)
                    .service(routine::routine_tasks),
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

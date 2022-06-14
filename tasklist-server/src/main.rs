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

fn insert_new_tasklist(name: &str) {
    use db::schema::{tasklist_partof, tasklists};

    let connection = db_connection().unwrap();
    let new_tasklist = db::model::insert::Tasklist {
        name,
        state: "not-started",
        routine_id: 0,
    };

    diesel::insert_into(tasklists::table)
        .values(&new_tasklist)
        .execute(&connection)
        .expect("Error inserting new tasklist");

    let tasklist_tasks = vec![
        db::model::insert::TasklistPartof {
            tasklist: 1,
            task: 1,
        },
        db::model::insert::TasklistPartof {
            tasklist: 1,
            task: 2,
        },
    ];

    diesel::insert_into(tasklist_partof::table)
        .values(&tasklist_tasks)
        .execute(&connection)
        .expect("Error inserting tasklist partof");
}

fn mark_task_done(search_name: &str) {
    use db::schema::tasks::dsl::*;

    let connection = db_connection().unwrap();
    let _task = tasks
        .filter(name.eq(search_name))
        .limit(1)
        .load::<db::model::Task>(&connection)
        .expect("Error loading tasks")
        .first()
        .expect("Couldn't find task")
        .id;

    // diesel::update(tasks.find(task))
    //     .set(done.eq(true))
    //     .execute(&connection)
    //     .expect("Couldn't find task");
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
                    // .service(routine::add_task)
                    // .service(routine::init)
                    .service(routine::get)
                    .service(routine::list)
                    .service(routine::new),
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

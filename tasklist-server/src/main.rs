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

mod model;
mod routine;
mod schema;
mod task;
mod tasklist;

diesel_migrations::embed_migrations!("migrations");

fn db_connection() -> Result<SqliteConnection, ConnectionError> {
    SqliteConnection::establish("tasklist.sqlite")
}

fn show_tasks() {
    use schema::tasks::dsl::*;

    let connection = db_connection().unwrap();
    let results = tasks
        .limit(5)
        .load::<model::Task>(&connection)
        .expect("Error loading tasks");

    println!("Displaying {} tasks", results.len());
    for task in results {
        println!("{} ({})", task.name, task.state.to_string());
    }
}

fn insert_new_task(name: &str) {
    use schema::tasks;

    let connection = db_connection().unwrap();
    let new_task = model::insert::Task {
        name,
        state: "not-started",
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(&connection)
        .expect("Error insert new task");
}

fn insert_new_tasklist(name: &str) {
    use schema::{tasklist_partof, tasklists};

    let connection = db_connection().unwrap();
    let new_tasklist = model::insert::Tasklist {
        name,
        state: "not-started",
        routine_id: 0,
    };

    diesel::insert_into(tasklists::table)
        .values(&new_tasklist)
        .execute(&connection)
        .expect("Error inserting new tasklist");

    let tasklist_tasks = vec![
        model::insert::TasklistPartof {
            tasklist: 1,
            task: 1,
        },
        model::insert::TasklistPartof {
            tasklist: 1,
            task: 2,
        },
    ];

    diesel::insert_into(tasklist_partof::table)
        .values(&tasklist_tasks)
        .execute(&connection)
        .expect("Error inserting tasklist partof");
}

fn insert_new_routine(name: &str) {
    use schema::routines;

    let connection = db_connection().unwrap();
    let new_routine = model::insert::Routine { name, model: 1 };

    diesel::insert_into(routines::table)
        .values(&new_routine)
        .execute(&connection)
        .expect("Error inserting new routine");
}

fn mark_task_done(search_name: &str) {
    use schema::tasks::dsl::*;

    let connection = db_connection().unwrap();
    let _task = tasks
        .filter(name.eq(search_name))
        .limit(1)
        .load::<model::Task>(&connection)
        .expect("Error loading tasks")
        .first()
        .expect("Couldn't find task")
        .id;

    // diesel::update(tasks.find(task))
    //     .set(done.eq(true))
    //     .execute(&connection)
    //     .expect("Couldn't find task");
}

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .compact()
        .init();

    let connection = db_connection().unwrap();
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();

    // insert_new_tasklist("list1");
    // insert_new_task("aaaaa");
    // insert_new_task("bbbbb");
    // insert_new_task("ccccc");
    // show_tasks();
    // mark_task_done("aaaaa");
    // show_tasks();
    insert_new_routine("routine123");

    let manager = ConnectionManager::<SqliteConnection>::new("tasklist.sqlite");
    let pool: DbPool = r2d2::Pool::builder().build(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(actix_web::middleware::NormalizePath::trim())
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/routine")
                    //         .service(routine::add_task)
                    //         .service(routine::get)
                    //         .service(routine::init)
                    .service(routine::list), //         .service(routine::new),
            )
            .service(web::scope("/task").service(task::get).service(task::put))
            .service(
                web::scope("/tasklist")
                    // .service(tasklist::delete_task)
                    .service(tasklist::get)
                    .service(tasklist::list), // .service(tasklist::new)
                                              // .service(tasklist::put),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

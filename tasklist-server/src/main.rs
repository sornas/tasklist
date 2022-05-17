use actix_web::{web, App, HttpServer};
use tracing::Level;
use tracing_actix_web::TracingLogger;

mod routine;
mod tasklist;

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
            .service(web::scope("/tasklist")
                .service(routine::get)
                .service(routine::new)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

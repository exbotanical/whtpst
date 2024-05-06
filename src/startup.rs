use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::{net::TcpListener, sync::Mutex};
use tracing_actix_web::TracingLogger;

use crate::{dao::Repository, routes};

pub fn run<R: Repository>(
    listener: TcpListener,
    db_pool: PgPool,
    repo: R,
) -> Result<Server, std::io::Error> {
    let repo = web::Data::new(Mutex::new(repo));
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health", web::get().to(routes::health))
            .route(
                "/paste",
                web::post().to(routes::create_paste_random_id::<R>),
            )
            .route("/paste/{id}", web::post().to(routes::create_paste::<R>))
            .route("/paste/{id}", web::get().to(routes::get_paste::<R>))
            // .route("/paste", web::post().to(routes::paste::<R>))
            .app_data(web::Data::clone(&repo))
            .app_data(web::Data::clone(&db_pool))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
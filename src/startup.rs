use actix_web::{dev::Server, web, App, HttpServer};
use std::{net::TcpListener, sync::Mutex};
use tracing_actix_web::TracingLogger;

use crate::{dao::Repository, routes};

pub fn run<R: Repository>(listener: TcpListener, repo: R) -> Result<Server, std::io::Error> {
    let repo = web::Data::new(Mutex::new(repo));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(routes::index))
            .route("/health", web::get().to(routes::health))
            .route("/paste", web::post().to(routes::create_paste_sans_id::<R>))
            .route("/paste/{id}", web::post().to(routes::create_paste::<R>))
            .route("/paste/{id}", web::get().to(routes::get_paste::<R>))
            .app_data(web::Data::clone(&repo))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

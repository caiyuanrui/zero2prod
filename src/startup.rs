use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;
use std::sync::Mutex;

use crate::routes::*;

pub fn run(listener: TcpListener, connection: PgConnection) -> std::io::Result<Server> {
    // wrap it in a smart pointer
    let connection = web::Data::new(Mutex::new(connection));
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subrcribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

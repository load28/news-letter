use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::io::Error;
use std::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub fn run(tcp_listener: TcpListener, db_pool: PgPool) -> Result<Server, Error> {
    let connection_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(connection_pool.clone())
    })
    .listen(tcp_listener)?
    .run();
    Ok(server)
}

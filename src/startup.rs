use actix_web::dev::Server;
use std::net::TcpListener;
use actix_web::{web, App, HttpServer};
use sqlx::{PgPool};


use crate::routes::{health_check, subscribe};

pub fn run(listner: TcpListener, db_pool: PgPool) -> Result<Server,std::io::Error> {

    let db_pool = web::Data::new(db_pool);
    let server  = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscription", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listner)? // Thats why there is a `?` here if the listen fails to bind to the listener the run function is not called, and the error is thrown one level up the call stack
    .run();
    Ok(server)
}


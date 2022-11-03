use std::net::TcpListener;

use actix_web::{get, web, App, HttpServer, HttpResponse,Responder};
use actix_web::dev::Server;
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!",)
}
async fn health_check() -> impl Responder {
    // This is where our logic will go
    HttpResponse::Ok()
}

#[derive(serde::Deserialize)]
struct FormData{
    email:String,
    name:String
}


async fn subscribe(_form:web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listner: TcpListener) -> Result<Server,std::io::Error> {
    let server  = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscription", web::post().to(subscribe))
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
    })
    .listen(listner)?
    .run();
    Ok(server)
}
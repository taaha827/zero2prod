use std::net::TcpListener;
// Rather then importing everything individually, RUST allows to  import multiple items from a crack using to some syntax 
// Like how jave script dose it that is the Curly BRacket Notation
use actix_web::{get, web, App, HttpServer, HttpResponse,Responder};
use actix_web::dev::Server;

// This is an example of attribute like macro
#[get("/hello/{name}")]
// This is an example in actix_web to extract data from the request, directly
// Actix web allows upto 10 items here, there are other options like FORM, PATH,QUERY,JSON
async fn greet(name: web::Path<String>) -> impl Responder {
    // Another Example of a simple macro that basically matches the attributes passed into it and produces code accordingly
    // #metaprogramming
    format!("Hello {name}!",)
}
async fn health_check() -> impl Responder {
    // This is where our logic will go
    HttpResponse::Ok()
}

// This is an example of a method like macro, that will use the  struct given to it and will
// Create serialize and deserialize attributes from it.
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
    .listen(listner)? // Thats why there is a `?` here if the listen fails to bind to the listener the run function is not called, and the error is thrown one level up the call stack
    .run();
    Ok(server)
}
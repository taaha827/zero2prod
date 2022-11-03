use std::net::TcpListener;

use zero2prod::run;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind address");
    run(listener)?.await
}


#[cfg(test)]
mod tests{
    // Here we would write internal tests for private sub routines 
}
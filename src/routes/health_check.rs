use actix_web::{ HttpResponse,Responder};

pub async  fn health_check() -> impl Responder {
    // This is where our logic will go
    HttpResponse::Ok()
}
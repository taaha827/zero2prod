// This is an example of a method like macro, that will use the  struct given to it and will
// Create serialize and deserialize attributes from it.
use actix_web::{ web, HttpResponse,};
use sqlx::{PgPool};
use chrono::Utc; 
use uuid::Uuid;
#[derive(serde::Deserialize)]
pub struct FormData{
    email:String,
    name:String
}
pub async fn subscribe(_form:web::Form<FormData>, _pool : web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    ).execute(_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            println!("Failed to execute query: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// This is an example of a method like macro, that will use the  struct given to it and will
// Create serialize and deserialize attributes from it.
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber", skip(_form, _pool),
    fields(
  subscriber_email = %_form.email, subscriber_name= %_form.name
) )]
pub async fn subscribe(_form: web::Form<FormData>, _pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&_pool,& _form)
    .await
    {
        Ok(_) =>  HttpResponse::Ok().finish(),
        Err(_err) => HttpResponse::InternalServerError().finish()
     
    }
}
#[tracing::instrument(
    name = "Saving new subscriber details in the database",
        skip(_form, _pool)
)]
pub async fn insert_subscriber(_pool: &PgPool, _form:&FormData) -> Result<(),sqlx::Error>{
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    )
    .execute(_pool)
    .await
    .map_err(|e|{
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())

}

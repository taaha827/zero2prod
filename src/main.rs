// Importing the Tcp Listnerer Structure
use sqlx::PgPool;
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::startup::run;
use secrecy::ExposeSecret;
use zero2prod::configuration::get_configurations;
// Using Declarative macro to technically create a main function that will run asynchrnously, as by default you can not have the main function
// Of a rust service set as async
use zero2prod::telementary::{get_subscriber, init_subscriber};

/*
Returning A Result Data type, remember that by default RUST treat lines of code as expression (i.e: Lines that perform an action and return a value, and the last line of a function is always implied as return statement)
*/
// Result<()> is used so that it shows that it will return OK signal without any value
#[actix_web::main]
// Or error if needed
async fn main() -> std::io::Result<()> {
    // Redirect all `log`'s events to our subscriber
    let subscriber = get_subscriber("zero2prod".into(), "info".into(),std::io::stdout);
    init_subscriber(subscriber);
    let mut configuration = get_configurations().expect("Failed to read configuration");
    configuration.database_settings.database_name = Uuid::new_v4().to_string();
    let connection_pool = PgPool::connect(&configuration.database_settings.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    // Bind returns a RESULT, so if we want to cater that we use expect, which will cause the code to exit if the bind function returns an error
    // Expect is not like catch just a way to catch an error and modify the error message, it does not allow to write a function per se.
    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))
        .expect("Failed to bind address");
    // ? allows to proceed only if there is no error, other wise throw the error back to who ever has called the function
    run(listener, connection_pool)?.await
}

#[cfg(test)]
mod tests {
    // Here we would write internal tests for private sub routines
}

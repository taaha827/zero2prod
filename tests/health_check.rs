use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configurations, DatabaseSettings};
use zero2prod::telementary::{get_subscriber, init_subscriber};
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    // We cannot assign the output of `get_subscriber` to a variable based on the value of `TEST_LOG` // because the sink is part of the type returned by `get_subscriber`, therefore they are not the
    // same type. We could work around it, but this is the most straight-forward way of moving forward. if std::env::var("TEST_LOG").is_ok() {
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});
#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &address.address))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=taaha827&email=taaha827%40gmail.com";
    let response = client
        .post(format!("{}/subscription",&address.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Could not send request");
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("select email,name FROM subscriptions",)
        .fetch_one(&address.db_pool)
        .await
        .expect("Failed To Fetch Subscription");

    assert_eq!(saved.email, "taaha827@gmail.com");
    assert_eq!(saved.name, "taaha827");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_bodies, invalid_error) in test_cases {
        let response = client
            .post(&format!("{}/subscription", &address.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_bodies)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            invalid_error
        );
    }
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let mut configuration = get_configurations().expect("Failed to read configuration");
    configuration.database_settings.database_name = Uuid::new_v4().to_string();
    let connection = PgPool::connect(
        &configuration
            .database_settings
            .connection_string()
            .expose_secret(),
    )
    .await
    .expect("Failed to connect to Postgres.");
    let listner = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listner.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listner, connection.clone()).expect("Could not bind to address"); // Launch the server as a background task
                                                                                                   // tokio::spawn returns a handle to the spawned future,
                                                                                                   // but we have no use for it here, hence the non-binding let let _ = tokio::spawn(server);
    let _ = tokio::spawn(server);
    let address = format!("http://127.0.0.1:{}", port);
    TestApp {
        address,
        db_pool: connection,
    }
}
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection =
        PgConnection::connect(&config.connection_string_without_db().expose_secret())
            .await
            .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");
    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}

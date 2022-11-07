use sqlx::{Connection, PgConnection, PgPool, Executor};
use std::net::TcpListener;
use zero2prod::configuration::{get_configurations, DatabaseSettings};
use uuid::Uuid;
#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &address))
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
    let configuration = get_configurations().expect("Could not load configuration File");
    let connection_string = configuration.database_settings.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Could Not connect to database");

    let body = "name=taaha827&email=taaha827%40gmail.com";
    let response = client
        .post(format!("{}/subscription", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Could not send request");
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("select email,name FROM subscriptions",)
        .fetch_one(&mut connection)
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
            .post(&format!("{}/subscription", &address))
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

async fn spawn_app() -> String {
    let mut configuration = get_configurations().expect("Failed to read configuration");
    configuration.database_settings.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database_settings).await;
    let connection = PgPool::connect(&configuration.database_settings.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let listner = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listner.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listner, connection).expect("Could not bind to address"); // Launch the server as a background task
                                                                                                   // tokio::spawn returns a handle to the spawned future,
                                                                                                   // but we have no use for it here, hence the non-binding let let _ = tokio::spawn(server);
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
    .await
            .expect("Failed to connect to Postgres");
        connection
    .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str()) .await
    .expect("Failed to create database.");
        // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string()) .await
    .expect("Failed to connect to Postgres."); sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database");
        connection_pool
    }
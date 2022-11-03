use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
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
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=taaha827&email=taaha827%40gmail.com";
    let response = client
        .post(format!("{}/subscription", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Could not send request");
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let address = spawn_app();
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

fn spawn_app() -> String {
    let listner = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listner.local_addr().unwrap().port();
    let server = zero2prod::run(listner).expect("Could not bind to address"); // Launch the server as a background task
                                                                              // tokio::spawn returns a handle to the spawned future,
                                                                              // but we have no use for it here, hence the non-binding let let _ = tokio::spawn(server);
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
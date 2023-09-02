use std::net::TcpListener;

// Spins up an isntance of the application and returns the address
fn spawn_app() -> String {
    // Binds the listener to a random port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Get the port assigned and start up a server instance
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Send Valid Response
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invlid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/suscription", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invlid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            404,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}

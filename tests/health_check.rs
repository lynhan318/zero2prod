use std::net::TcpListener;

use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    let server = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", server))
        .send()
        .await
        .expect("Failed to execute rquest");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    let app_address = spawn_app();
    let test_cases = vec![
        ("name=abc", "missing the name"),
        ("email=abc", "missing the email"),
        ("", "missing both name and email"),
    ];
    let client = reqwest::Client::new();

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .body(invalid_body)
            .header("Content-type", "application/x-www-form-urlencoded")
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when payload was {}.",
            error_message
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to spawn our app");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

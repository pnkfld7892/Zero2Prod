use std::net::TcpListener;

use sqlx::{Connection, PgConnection};
use zero2prod::configuration::get_configuration;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::startup::run(listener).expect("Failed to Bind Address");
    let _ = tokio::spawn(server);

    //return full application address to caller
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    //assertion
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_valid_form() {
    //arrange
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read config!");
    let connection_string = configuration.database.connection_string();
    let mut pgConn = PgConnection::connect(&connection_string).await.expect("Cannot Connect to DB");
    let client = reqwest::Client::new();

    //act
    let body = "name=eli%20schwarz&email=eschwarz0%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute post request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("Select email, name FROM subscriptions",)
        .fetch_one(&mut pgConn)
        .await
        .expect("Failed to retrieve sql value.");

    assert_eq!(saved.email,"eschwarz0@gmail.com");
    assert_eq!(saved.name,"eli schwarz");
}

#[tokio::test]
async fn subscribe_returns_400_when_data_missing() {
    //arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        //act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlenclosed")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute post request");

        //assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "API did not fail with 400 bad request when payload was {}",
            error_message
        );
    }
}

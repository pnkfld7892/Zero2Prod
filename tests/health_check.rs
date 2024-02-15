use std::net::TcpListener;

use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid;
use zero2prod::configuration::{get_configuration,DataBaseSettings};

struct TestApp {
    pub address: String,
    pub db_pool: PgPool
}

async fn spawn_app() -> TestApp{
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = uuid::Uuid::new_v4().to_string();
    // let connection_pool = PgPool::connect(&configuration.database.connection_string())
    //     .await
    //     .expect("Failed to connect to db");
    let connection_pool = configure_database(&configuration.database).await;

    let server = zero2prod::startup::run(listener, connection_pool.clone()).expect("Failed to build server");

    let _ = tokio::spawn(server);

    //return full application address to caller
    TestApp{
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DataBaseSettings) -> PgPool{
    //create db
    let mut connection = PgConnection::connect(
            &config.connection_string_without_db()
        )
        .await
        .expect("Failed to connect to postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}""#, config.database_name).as_str())
        .await
        .expect("Failed To Create DB");

    //do the miggies
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed To make Pg pool and connect!");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool

}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &app.address))
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
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    //act
    let body = "name=eli%20schwarz&email=eschwarz0%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute post request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("Select email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to retrieve sql value.");

    assert_eq!(saved.email,"eschwarz0@gmail.com");
    assert_eq!(saved.name,"eli schwarz");
}

#[tokio::test]
async fn subscribe_returns_400_when_data_missing() {
    //arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        //act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
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

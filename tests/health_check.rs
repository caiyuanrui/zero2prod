use reqwest::StatusCode;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::{borrow::Borrow, net::TcpListener};
use uuid::Uuid;

use zero2prod::{
    configuration::{self, DatabaseSettings},
    startup::run,
};

#[derive(Debug)]
pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{port}");

    let mut config = configuration::get_configuration().unwrap();
    config.database.database_name = Uuid::new_v4().to_string();
    let conn_pool = configure_database(&config.database).await;

    let server = run(listener, conn_pool.clone()).expect("Failed to bind address.");
    tokio::spawn(server);

    TestApp {
        address,
        pool: conn_pool,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut conn = PgConnection::connect(&config.connction_string_without_db())
        .await
        .expect("Failed to connect to Postgres.");
    conn.execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let conn_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&conn_pool)
        .await
        .expect("Failed to migrate the database.");

    conn_pool
}

#[tokio::test]
async fn health_check_success() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to excute request.");

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_email_and_name() {
    // Arange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let res = client
        .post(format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to excute request.");

    // Assert
    assert_eq!(res.status(), StatusCode::OK);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(app.pool.borrow())
        .await
        .expect("Failed to fetch saved subscriptions.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = [
        ("", "missing both name and email"),
        ("name=le%20guinm", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
    ];

    for (body, err_msg) in test_cases {
        let res = client
            .post(format!("{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to excute request.");

        assert_eq!(
            res.status(),
            StatusCode::BAD_REQUEST,
            "The API should return 400 when the payload is {}.",
            err_msg
        );
    }
}

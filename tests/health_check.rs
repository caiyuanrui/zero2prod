use reqwest::StatusCode;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::configuration::{self, get_configuration};

#[tokio::test]
async fn health_check_success() {
    let addr = spawn_app().await;
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{addr}/health_check"))
        .send()
        .await
        .expect("Failed to excute request.");

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_email_and_name() {
    // Arrange
    let addr = spawn_app().await;
    let config = get_configuration().expect("Failed to read configuration.");
    let connect_string = config.database.connection_string();

    let mut connection = PgConnection::connect(&connect_string)
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let res = client
        .post(format!("{addr}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to excute request.");

    assert_eq!(res.status(), StatusCode::OK);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscriptions.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_form_data() {
    let addr = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = [
        ("", "missing both name and email"),
        ("name=le%20guinm", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
    ];

    for (body, err_msg) in test_cases {
        let res = client
            .post(format!("{addr}/subscriptions"))
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

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let addr = listener.local_addr().unwrap().to_string();
    let config = configuration::get_configuration().unwrap();
    let conn = PgConnection::connect(&config.database.connection_string())
        .await
        .unwrap();
    let server = zero2prod::startup::run(listener, conn).expect("Failed to bind address.");
    tokio::spawn(server);
    format!("http://{addr}")
}

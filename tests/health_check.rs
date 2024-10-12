use reqwest::StatusCode;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_success() {
    let addr = spawn_app();
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{addr}/health_check"))
        .send()
        .await
        .expect("Failed to excute request.");

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.content_length(), Some(0));
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let addr = listener.local_addr().unwrap().to_string();
    let server = zero2prod::run(listener).expect("Failed to bind address.");
    tokio::spawn(server);
    format!("http://{addr}")
}

use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telementry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration.");
    let lst = TcpListener::bind(format!("127.0.0.1:{}", config.application_port))?;

    let db_pool = PgPool::connect(config.database.connection_string().expose_secret())
        .await
        .expect("Failed to create a connection pool");
    run(lst, db_pool)?.await
}

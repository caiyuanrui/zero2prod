use std::net::TcpListener;

use sqlx::{Connection, PgConnection};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to read configuration.");
    let lst = TcpListener::bind(format!("127.0.0.1:{}", config.application_port))?;
    let conn = PgConnection::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    run(lst, conn)?.await
}

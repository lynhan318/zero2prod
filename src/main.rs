use std::net::TcpListener;

use sqlx::{Connection, PgConnection, PgPool};
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to read configuration");

    let listener =
        TcpListener::bind(config.application.server_url()).expect("Failed to bind address");

    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    run(listener, connection_pool)?.await
}

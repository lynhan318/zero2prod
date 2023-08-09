use secrecy::ExposeSecret;
use std::net::TcpListener;

use sqlx::PgPool;

use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration");

    let listener =
        TcpListener::bind(config.application.server_url()).expect("Failed to bind address");

    let connection_pool =
        PgPool::connect_lazy(&config.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgres.");

    run(listener, connection_pool)?.await
}

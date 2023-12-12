use falcon_rust::configuration::get_configuration;
use falcon_rust::startup::run;
use falcon_rust::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("falcon-external".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // panic if we can't read config
    let configuration = get_configuration().expect("Failed to read config");
    let connection_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to database");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    // Bubble up the io::Error if we failed to bind address
    run(listener, connection_pool)?.await
}

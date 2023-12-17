use falcon_rust::configuration::get_configuration;
use falcon_rust::startup::run;
use falcon_rust::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("falcon-external".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // panic if we can't read config
    let configuration = get_configuration().expect("Failed to read config");

    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    // Bubble up the io::Error if we failed to bind address
    run(listener, connection_pool)?.await
}

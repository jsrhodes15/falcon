use falcon_rust::configuration::get_configuration;
use falcon_rust::startup::Application;
use falcon_rust::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("falcon-external".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // panic if we can't read config
    let configuration = get_configuration().expect("Failed to read config");

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}

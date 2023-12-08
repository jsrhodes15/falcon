use falcon_rust::startup::run;
use falcon_rust::configuration::get_configuration;
use std::net::TcpListener;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
   // panic if we can't read config
    let configuration = get_configuration().expect("Failed to read config");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    // Bubble up the io::Error if we failed to bind address
    run(listener)?.await
}

use std::net::TcpListener;
use falcon_rust::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to port 8000");
    // Bubble up the io::Error if we failed to bind address
    run(listener)?.await
}

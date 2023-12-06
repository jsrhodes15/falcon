use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // act
    let response = client
        .get(&format!("{}/health", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let port = listener.local_addr().unwrap().port();
    let server = falcon_rust::run(listener).expect("Failed to bind address");
    // Launch server as background task
    // tokio::spawn returns a handle to the spawned Future,
    // but we have no use for it here, hence the non-binding let _
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
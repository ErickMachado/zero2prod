use std::net::TcpListener;
use zero2prod::run;

fn spawn_app() -> String {
    let address = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = address.local_addr().unwrap().port();
    let server = run(address).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

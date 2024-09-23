//! tests/health_check.rs

// The test covers the full range of properties we are interested to check:
// • the health check is exposed at /health_check;
// • the health check is behind a GET method;
// • the health check always returns a 200;
// • the health check’s response has no body

use std::net::TcpListener;

// `actix_web::test` is the testing equivalent of `actix_web::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // Arrange
    // spawn_app().await.expect("Failed to spawn our app.");
    // No .await, no .expect
    let address = spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = email_newsletter::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}

// Launch our application in the background ~somehow~
// async fn spawn_app() -> Result<(), std::io::Error> {
// if we fail to perform the required setup we can just panic and crash
// all the things.
// fn spawn_app() {
//     // email_newsletter::run().await
//     let server = email_newsletter::run("127.0.0.1:0").expect("Failed to bind address");
//     // Launch the server as a background task
//     // tokio::spawn returns a handle to the spawned future,
//     // but we have no use for it here, hence the non-binding let
//     let _ = tokio::spawn(server);
// }
//

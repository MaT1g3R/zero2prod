use std::{io, net::TcpListener};

use rocket::tokio;
use zero2prod::app;

#[rocket::async_test]
async fn health_check_works() {
    let port = spawn_app().expect("Failed to launch app.");

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://127.0.0.1:{}/health_check", port))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> io::Result<u16> {
    let port = TcpListener::bind("127.0.0.1:0")?.local_addr()?.port();
    let _ = tokio::spawn(app(port).launch());
    Ok(port)
}

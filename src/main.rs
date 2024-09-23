use email_newsletter::run;
use std::net::TcpListener;

// #[actix_web::main]
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run(listener)?.await
    // Ok(())
}

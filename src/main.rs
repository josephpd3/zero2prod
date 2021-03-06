//! main.rs

use std::net::TcpListener;

use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind to port 8000.");
    println!("Listening at 127.0.0.1:8000...");
    run(listener)?.await
}

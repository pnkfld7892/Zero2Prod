use std::net::TcpListener;

use zero2prod::run;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to get listener");
    let server = run(listener).expect("Failed to bind address");
    tokio::spawn(server).await?
}

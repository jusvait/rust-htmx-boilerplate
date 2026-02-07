mod error;
mod index;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(index::index));

    match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => axum::serve(listener, app).await.unwrap(),
        Err(err) => println!("{}", err),
    }

    Ok(())
}

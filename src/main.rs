use std::{env::var, net::SocketAddr};

use axum::serve;
use dotenvy::dotenv;
use file_sharing::app::app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let app = app()?;

    let addr: SocketAddr = var("SOCKET_ADDR")
        .expect("SOCKET_ADDR env not set")
        .parse()?;
    let listener = TcpListener::bind(addr).await?;

    serve(listener, app.into_make_service()).await?;

    println!("Server runnnig at: {addr}");

    Ok(())
}

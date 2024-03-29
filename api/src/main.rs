mod app;
mod entity;

use app::app;
use tokio::{net::TcpListener, signal::unix};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = app();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let mut sigterm = unix::signal(unix::SignalKind::terminate()).unwrap();
    let _ = sigterm.recv().await;
}

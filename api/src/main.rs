mod app;
mod entity;

use std::{
    env,
    net::{Ipv4Addr, SocketAddrV4},
};

use app::app;
use tokio::{net::TcpListener, signal::unix};

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .expect("port not set.")
        .parse::<u16>()
        .unwrap();
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
    let listener = TcpListener::bind(addr).await.unwrap();
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

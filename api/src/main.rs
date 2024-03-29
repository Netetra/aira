mod app;
mod entity;

use app::app;
use sea_orm::{ConnectOptions, Database, DbConn, DbErr};
use std::env;
use tokio::{net::TcpListener, signal::unix};

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE_URL").unwrap();
    let db = connect_db(&db_url).await.unwrap();
    let app = app(db).await;
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn connect_db(db_url: &str) -> Result<DbConn, DbErr> {
    let option = ConnectOptions::new(db_url);
    Database::connect(option).await
}

async fn shutdown_signal() {
    let mut sigterm = unix::signal(unix::SignalKind::terminate()).unwrap();
    let _ = sigterm.recv().await;
}

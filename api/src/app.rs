use axum::Router;
use sea_orm::prelude::DbConn;

pub async fn app(db: DbConn) -> Router {
    Router::new().with_state(db)
}

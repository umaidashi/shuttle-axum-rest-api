use axum::{routing::get, Router};
use sqlx::{Executor, PgPool};



async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
) -> shuttle_axum::ShuttleAxum {
    db.execute(include_str!("../migrations.sql")).await.unwrap();

    let state = AppState { db };

    let router = Router::new().route("/", get(hello_world)).with_state(state);

    Ok(router.into())
}

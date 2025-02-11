use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, PgPool};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

impl AppState {
    fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] db: PgPool) -> shuttle_axum::ShuttleAxum {
    db.execute(include_str!("../migrations.sql")).await.unwrap();

    let state = AppState::new(db);

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/users", get(retrieve_all_records).post(create_record))
        .route(
            "/users/{id}",
            get(retrieve_record_by_id)
                .delete(delete_record_by_id)
                .put(update_record_by_id),
        )
        .with_state(state);

    Ok(router.into())
}

#[derive(sqlx::FromRow, Serialize)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

async fn retrieve_all_records(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let res = match sqlx::query_as::<_, User>("select * from users")
        .fetch_all(&state.db)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    Ok(Json(res))
}

async fn retrieve_record_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let res = match sqlx::query_as::<_, User>("SELECT * FROM USERS WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    Ok(Json(res))
}

#[derive(Deserialize)]
pub struct UserSubmission {
    name: String,
    age: i32,
}

async fn create_record(
    State(state): State<AppState>,
    Json(json): Json<UserSubmission>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query("INSERT INTO USERS (name, age) VALUES ($1, $2)")
        .bind(json.name)
        .bind(json.age)
        .execute(&state.db)
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error while inserting a record: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}

async fn delete_record_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query_as::<_, User>("DELETE FROM USERS WHERE ID = $1")
        .bind(id)
        .fetch_all(&state.db)
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error while deleting a record: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct UpdateRecord {
    name: Option<String>,
    age: Option<i32>,
}

async fn update_record_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(json): Json<UserSubmission>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query(
        "UPDATE USERS (name, age)
            SET name = (case when $1 is not null then $1 else name end),
            age = (case when $2 is not null then $2 else age end)
            WHERE
            id = $3",
    )
    .bind(json.name)
    .bind(json.age)
    .bind(id)
    .execute(&state.db)
    .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error while inserting a record: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}

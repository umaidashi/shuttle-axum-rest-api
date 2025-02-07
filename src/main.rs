use axum::{routing::get, Router};

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world));

    Ok(router.into())
}

// this is a function that returns a static string
// all functions used as endpoints must return a HTTP-compatible response
async fn hello_world() -> &'static str {
    "Hello world!"
}

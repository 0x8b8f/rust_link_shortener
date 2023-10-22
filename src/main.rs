use axum::{response::Html, response::IntoResponse, routing::get, routing::post, routing::delete, Router, Server};

pub async fn create() -> impl IntoResponse {
    "create"
}

pub async fn go() -> impl IntoResponse {
    "go"
}

pub async fn remove() -> impl IntoResponse {
    "remove"
}

#[tokio::main]
pub async fn main() {
    let router: Router = Router::new().route("/", get(|| async { Html("<h1>Url Shortener</h1>") }))
        .route("/create", post(create))
        .route("/go", get(go))
        .route("/remove", delete(remove));

    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

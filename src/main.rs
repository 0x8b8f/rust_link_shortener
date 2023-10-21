use axum::{response::Html, routing::get, Router, Server};

#[tokio::main]
pub async fn main() {
    let router: Router = Router::new().route("/", get(|| async { Html("<h1>Url Shortener</h1>") }));

    Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

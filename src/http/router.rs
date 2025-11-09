use axum::{Router, response::Html, routing::get};
use std::fs;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/health", get(|| async { "OK" }))
}

// @todo implement logs
pub async fn home() -> Html<String> {
    let content = fs::read_to_string("resources/views/welcome.html.tera")
        .unwrap_or_else(|_| {
            eprintln!("Error reading template");
            String::from("<h1>Error loading page</h1>")
        });

    Html(content)
}

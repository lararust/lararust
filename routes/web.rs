use crate::view::view;
use crate::{ctx, view::view};
use axum::{Router, response::Html, routing::get};
use tera::Context;

/// Registers routes
pub fn routes() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/health", get(|| async { "OK" }))
}

/// Example route rendering a LaraBlade template
pub async fn home() -> Html<String> {
    // Build context using the ctx! macro
    let ctx = ctx! {
        "name" => "Vinícius Rech",
        "admin" => true,
        "age" => 32,
        "roles" => vec!["developer", "maintainer"]
    };

    view("welcome", ctx)
}

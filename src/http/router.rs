use std::sync::Arc;

use axum::{http::Request, response::Response};

pub type Handler = Arc<dyn Fn(Request) -> Response + Send + Sync + 'static>;

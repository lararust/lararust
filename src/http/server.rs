use std::sync::Arc;

use axum::Router;

pub struct Server {
    address: String,
    router: Arc<Router>,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }
}

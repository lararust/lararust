use crate::http::server::run_server;

pub async fn serve_command() {
    run_server().await;
}

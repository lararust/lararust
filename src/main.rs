mod cli;
mod http;
mod prelude;
mod support;

use cli::run_cli;

#[tokio::main]
pub async fn main() {
    run_cli().await;
}

mod app;
mod cli;
mod http;
mod prelude;
mod support;
#[macro_use]
mod view;

use crate::cli::run_cli;

#[tokio::main]
pub async fn main() {
    run_cli().await;
}

use crate::app::run_dev_server;

pub async fn serve_command() {
    // The handcrafted server is synchronous/blocking for now.
    // Running it on the current thread keeps behavior identical
    // between `cargo run` and `cargo run -- serve`.
    run_dev_server();
}

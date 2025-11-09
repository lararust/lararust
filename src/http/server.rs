use colored::*;
use std::{env, net::SocketAddr, process};
use tokio::net::TcpListener;

use crate::prelude::{load_env, routes};

pub async fn run_server() {
    load_env();

    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let app = routes();

    let port = match env::var("APP_PORT") {
        Ok(port) => port,
        Err(_) => {
            let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            println!(
                "{}",
                format!(
                    "[WARN] PORT environment variable not set, using default port 8080 at {}",
                    time
                )
                .yellow()
            );
            "8080".to_string()
        }
    };

    const LOCALHOST: [u8; 4] = [127, 0, 0, 1];

    // @todo implement logging
    let port_num = match port.parse::<u16>() {
        Ok(port) => port,
        Err(_) => {
            eprintln!("[ERROR] Invalid port number: '{}'", port);
            process::exit(1);
        }
    };

    let address = SocketAddr::from((LOCALHOST, port_num));

    let listener = match TcpListener::bind(&address).await {
        Ok(listener) => {
            println!(
                "{}",
                format!(
                    "[INFO] Server bound to {} at {}",
                    address,
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
                )
                .green()
            );
            listener
        }
        Err(error) => {
            eprintln!("[ERROR] Failed to bind to {}: {}", address, error);
            eprintln!("[TIP] Check if:");
            eprintln!("      - Port {} is available", port_num);
            eprintln!("      - No other service is using this port");
            eprintln!("      - You have permission to bind to this address");
            process::exit(1);
        }
    };

    if let Err(error) = axum::serve(listener, app).await {
        eprintln!("[ERROR] Server runtime error: {}", error);
        eprintln!(
            "[ERROR] Server shutting down at {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        );
        process::exit(1);
    }
}

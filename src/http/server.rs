/**
 * Handcrafted HTTP server built on `std::net::TcpListener`.
 *
 * This module provides a simple, synchronous HTTP server that spawns a thread
 * per connection. It's designed for early-stage development and will be replaced
 * with async/hyper-based infrastructure in future milestones.
 */
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use crate::http::requests::Request;
use crate::http::response::Response;
use crate::prelude::Router;

/**
 * HTTP server that binds to a TCP address and dispatches requests to a Router.
 *
 * The server is synchronous and spawns a new thread for each incoming connection.
 * This is simple but not production-ready; future versions will use async I/O
 * with Tokio and hyper/axum.
 *
 * # Example
 * ```
 * use lararust::http::{server::Server, router::Router, response::Response};
 *
 * let mut router = Router::new();
 * router.get("/", |_req| Response::text("Hello!"));
 *
 * let server = Server::new("127.0.0.1:8080", router);
 * server.run(); // Blocks forever
 * ```
 */
pub struct Server {
    address: String,
    router: Arc<Router>,
}

impl Server {
    /**
     * Creates a new server bound to the given address with the provided router.
     *
     * # Arguments
     * * `address` - Bind address in the format "IP:PORT" (e.g., "127.0.0.1:8080")
     * * `router` - Router containing all registered routes
     *
     * # Example
     * ```
     * let server = Server::new("0.0.0.0:3000", router);
     * ```
     */
    pub fn new(address: &str, router: Router) -> Self {
        Self {
            address: address.to_string(),
            router: Arc::new(router),
        }
    }

    /**
     * Starts the HTTP server and listens for incoming connections.
     *
     * This method **blocks forever** and never returns. Each incoming connection
     * spawns a new thread to handle the request independently.
     *
     * # Panics
     * Panics if the address cannot be bound (e.g., port already in use or permission denied).
     *
     * # Example
     * ```
     * server.run(); // Blocks until process is killed
     * ```
     */
    pub fn run(&self) {
        let listener = TcpListener::bind(&self.address).expect("Failed to bind address");

        println!("Lararust server running at http://{}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let router = Arc::clone(&self.router);
                    thread::spawn(move || {
                        handle_client(stream, router);
                    });
                }
                Err(err) => {
                    eprintln!("Failed to accept connection: {}", err);
                }
            }
        }
    }
}

/**
 * Handles a single client connection by reading the request, dispatching to the router,
 * and writing the response.
 *
 * # Special Handling
 * - HEAD requests automatically have their response bodies stripped (per HTTP spec)
 * - Malformed requests receive a 400 Bad Request response
 * - Read/write errors are logged to stderr but don't crash the server
 *
 * # Buffer Size
 * Currently reads up to 4KB per request. Larger requests will be truncated.
 * Future versions will support chunked transfer encoding and streaming.
 */
fn handle_client(mut stream: TcpStream, router: Arc<Router>) {
    let mut buffer = [0u8; 4096];

    match stream.read(&mut buffer) {
        Ok(bytes_read) if bytes_read > 0 => {
            let raw = String::from_utf8_lossy(&buffer[..bytes_read]);

            if let Some(request) = Request::from_raw(&raw) {
                let is_head = request.method == crate::http::requests::Method::HEAD;
                let response = router.dispatch(request);

                // For HEAD requests, send headers only (no body)
                let response_bytes = if is_head {
                    response.to_http_bytes_head_only()
                } else {
                    response.to_http_bytes()
                };

                if let Err(err) = stream.write_all(&response_bytes) {
                    eprintln!("Failed to write response: {}", err);
                }
            } else {
                // Malformed request
                let response_bytes = Response::new(400, "Bad Request").to_http_bytes();
                let _ = stream.write_all(&response_bytes);
            }
        }
        _ => {
            // Ignore empty/failed reads
        }
    }
}

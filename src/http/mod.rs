/**
 * Handcrafted HTTP stack for LaraRust.
 *
 * This module provides a lightweight, from-scratch HTTP implementation built on
 * `std::net::TcpListener`. It includes request parsing, response building, routing,
 * and a basic TCP server.
 *
 * # Architecture
 *
 * - [`requests`] - HTTP request parsing and method definitions
 * - [`response`] - Response builder with fluent API
 * - [`router`] - Route registration and request dispatching
 * - [`server`] - TCP listener and connection handling
 *
 * # Design Philosophy
 *
 * This handcrafted stack exists to validate Laravel-esque APIs before committing
 * to heavier dependencies like `hyper` or `axum`. Once the API surface stabilizes,
 * we'll migrate to production-ready infrastructure.
 *
 * # Example
 *
 * ```rust
 * use lararust::http::{response::Response, router::Router, server::Server};
 *
 * let mut router = Router::new();
 *
 * router
 *     .get("/", |_req| Response::html("<h1>Welcome</h1>"))
 *     .get("/health", |_req| Response::text("OK"))
 *     .post("/echo", |req| {
 *         let body = String::from_utf8_lossy(&req.body);
 *         Response::text(&body)
 *     });
 *
 * let server = Server::new("127.0.0.1:8080", router);
 * server.run();
 * ```
 *
 * # Current Limitations
 *
 * - Thread-per-connection (not scalable)
 * - 4KB request buffer limit
 * - Exact path matching only (no parameters yet)
 * - No middleware support
 * - Synchronous handlers only
 *
 * See the [HTTP module README](./README.md) for detailed documentation.
 */
pub mod requests;
pub mod response;
pub mod router;
pub mod server;

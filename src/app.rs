use crate::ctx;
use crate::http::{response::Response, server::Server};
use crate::prelude::{Router, app_port, load_env};
use crate::view::view as render_view;

/**
 * Bootstraps the sample router and starts the handcrafted HTTP server.
 *
 * This function demonstrates all supported HTTP methods with proper semantics:
 * - GET/POST/PUT/PATCH/DELETE: Process request bodies and return full responses
 * - HEAD: Returns headers only (body automatically stripped by server)
 * - OPTIONS: Returns allowed methods/headers for CORS preflight
 *
 * Keeping this logic in one place ensures `main` and the CLI `serve`
 * command stay perfectly aligned.
 */
pub fn run_dev_server() {
    load_env();

    let router = sample_router();
    let address = format!("127.0.0.1:{}", app_port());

    let server = Server::new(&address, router);
    server.run();
}

/**
 * Configures sample routes demonstrating all HTTP methods.
 *
 * Routes are chained using the fluent builder pattern. All methods return
 * `&mut Router` to enable chaining.
 */
fn sample_router() -> Router {
    let mut router = Router::new();

    router
        // Standard routes with view rendering
        .get("/", |_request| {
            let ctx = ctx! {
                "admin" => true,
                "framework" => "LaraRust",
            };

            render_view("welcome", ctx)
        })
        .get("/health", |_request| Response::text("OK"))
        .get("/hello", |_request| Response::text("Hello from Lararust Router!"))
        // Body-processing methods (POST, PUT, PATCH, DELETE)
        .post("/post", |request| {
            let body = request.body;
            let body_str = String::from_utf8_lossy(&body);
            Response::text(&format!("Hello, {}!", body_str))
        })
        .delete("/delete", |request| {
            let body = request.body;
            let body_str = String::from_utf8_lossy(&body);
            Response::text(&format!("Deleted: {}", body_str))
        })
        .put("/put", |request| {
            let body = request.body;
            let body_str = String::from_utf8_lossy(&body);
            Response::text(&format!("Put: {}", body_str))
        })
        .patch("/patch", |request| {
            let body = request.body;
            let body_str = String::from_utf8_lossy(&body);
            Response::text(&format!("Patched: {}", body_str))
        })
        // HEAD: Returns headers only (body stripped by server per HTTP spec)
        .head("/head", |_request| Response::new(200, ""))
        // OPTIONS: CORS preflight and capability discovery
        .options("/options", |_request| {
            Response::new(200, "")
                .with_header("Allow", "GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS")
                .with_header(
                    "Access-Control-Allow-Methods",
                    "GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS",
                )
                .with_header("Access-Control-Allow-Headers", "Content-Type, Authorization")
        });

    router
}

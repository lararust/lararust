# HTTP Module

The LaraRust HTTP module provides a handcrafted, from-scratch HTTP stack built on `std::net::TcpListener`. This lightweight implementation allows us to iterate on Laravel-esque APIs before graduating to heavier dependencies like `hyper` or `axum`.

## Architecture

The module consists of four core components:

```
src/http/
├── requests.rs   # HTTP request parsing and Method enum
├── response.rs   # HTTP response building with fluent API
├── router.rs     # Route registration and dispatching
└── server.rs     # TCP listener and connection handling
```

## Components

### Request (`requests.rs`)

Parses raw HTTP requests into a structured format.

**Supported Methods:**
- `GET`, `POST`, `PUT`, `PATCH`, `DELETE`, `OPTIONS`, `HEAD`

**Structure:**
```rust
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}
```

**Parsing:**
- Extracts method, path, and HTTP version from the request line
- Parses headers into a case-sensitive HashMap
- Captures the request body as raw bytes

### Response (`response.rs`)

Builds HTTP responses with a fluent, chainable API.

**Core Methods:**
```rust
Response::new(status_code: u16, body: impl Into<Vec<u8>>) -> Self
Response::text(text: &str) -> Self                              // 200 with text/plain
Response::html(body: impl Into<Vec<u8>>) -> Self                // 200 with text/html
Response::not_found() -> Self                                   // 404
Response::with_header(key: &str, value: &str) -> Self           // Chainable header setter
```

**Serialization:**
- `to_http_bytes()` - Full HTTP/1.1 response with headers and body
- `to_http_bytes_head_only()` - Headers only (for HEAD requests, per HTTP spec)

**Default Headers:**
- `Content-Type: text/plain; charset=utf-8` (overridable with `with_header`)
- `Content-Length: <body_length>`
- `Connection: close`

### Router (`router.rs`)

Registers routes and dispatches requests to handlers.

**Route Registration:**
```rust
router.get(path, handler)      // GET requests
router.post(path, handler)     // POST requests
router.put(path, handler)      // PUT requests
router.patch(path, handler)    // PATCH requests
router.delete(path, handler)   // DELETE requests
router.head(path, handler)     // HEAD requests (response body not sent)
router.options(path, handler)  // OPTIONS requests (CORS preflight)
```

**Handler Signature:**
```rust
Fn(Request) -> Response + Send + Sync + 'static
```

**Dispatching:**
- Exact match on `method` + `path`
- Returns `Response::not_found()` if no route matches
- No path parameters or wildcard support yet (planned for future milestones)

**Method Chaining:**
All route registration methods return `&mut Self`, enabling fluent chaining:

```rust
router
    .get("/", handler_home)
    .get("/health", handler_health)
    .post("/api/users", handler_create_user);
```

### Server (`server.rs`)

Binds to a TCP address and handles incoming connections.

**Usage:**
```rust
let server = Server::new("127.0.0.1:8080", router);
server.run(); // Blocks forever, listening for connections
```

**Connection Handling:**
- Each connection spawns a new thread (simple but functional for dev)
- Reads up to 4KB from the TCP stream
- Parses the raw bytes into a `Request`
- Dispatches to the `Router`
- Special handling: HEAD requests automatically strip response bodies

**Error Handling:**
- Malformed requests receive `400 Bad Request`
- Connection errors are logged to stderr but don't crash the server

## HTTP Method Semantics

### Standard Body-Processing Methods

**GET, POST, PUT, PATCH, DELETE** - Process request bodies and return full responses:

```rust
router.post("/users", |request| {
    let body = request.body;
    let body_str = String::from_utf8_lossy(&body);
    Response::text(&format!("Created user: {}", body_str))
});
```

### HEAD - Metadata Only

**HEAD** returns the same headers as GET would, but **no response body**. Use it to check if a resource exists or get metadata without downloading content:

```rust
router.head("/users/123", |_request| {
    // Body is ignored and won't be sent to client
    Response::new(200, "")
});
```

The server automatically strips the body from HEAD responses via `to_http_bytes_head_only()`.

**Use Cases:**
- Check if a resource exists (200 vs 404)
- Get `Content-Length` without downloading the resource
- Validate links in web crawlers

### OPTIONS - Capability Discovery

**OPTIONS** returns which HTTP methods and headers are allowed, primarily for CORS preflight requests:

```rust
router.options("/api/users", |_request| {
    Response::new(200, "")
        .with_header("Allow", "GET, POST, PUT, DELETE")
        .with_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE")
        .with_header("Access-Control-Allow-Headers", "Content-Type, Authorization")
});
```

**Use Cases:**
- CORS preflight requests from browsers
- API capability discovery
- Documenting allowed operations

## Example: Complete Server Setup

```rust
use lararust::{
    http::{response::Response, server::Server},
    prelude::{Router, app_port, load_env},
};

fn main() {
    load_env();

    let mut router = Router::new();

    router
        .get("/", |_req| Response::html("<h1>Welcome</h1>"))
        .get("/health", |_req| Response::text("OK"))
        .post("/echo", |req| {
            let body_str = String::from_utf8_lossy(&req.body);
            Response::text(&body_str)
        })
        .head("/ping", |_req| Response::new(200, ""))
        .options("/api", |_req| {
            Response::new(200, "")
                .with_header("Allow", "GET, POST, OPTIONS")
        });

    let address = format!("127.0.0.1:{}", app_port());
    Server::new(&address, router).run();
}
```

## Current Limitations

- **No async handlers** - All handlers are synchronous (Tokio runtime exists but isn't used here)
- **Thread-per-connection** - Simple but not scalable; will migrate to async when hyper/axum is integrated
- **4KB request buffer** - Larger requests will be truncated
- **Exact path matching only** - No route parameters (`/users/:id`) or wildcards yet
- **No middleware** - Planned for future milestones
- **Single-threaded router** - Uses `Arc<Router>` but doesn't benefit from parallelism yet

## Future Roadmap

1. **Path parameters** - `/users/:id` style routing
2. **Middleware** - CORS, logging, auth, rate limiting
3. **Async handlers** - Leverage Tokio runtime
4. **Hyper/Axum integration** - Graduate to production-ready HTTP stack
5. **WebSocket support** - Real-time bidirectional communication
6. **HTTP/2 and HTTP/3** - Modern protocol support

## Testing

```bash
# Start the server
cargo run

# Test GET
curl http://127.0.0.1:8080/health

# Test POST
curl -X POST http://127.0.0.1:8080/echo -d "Hello, LaraRust!"

# Test HEAD (note: -I is preferred over -X HEAD)
curl -I http://127.0.0.1:8080/ping

# Test OPTIONS
curl -X OPTIONS http://127.0.0.1:8080/api -v
```

## Design Philosophy

This handcrafted HTTP stack exists to:

1. **Validate Laravel-esque APIs** before committing to external dependencies
2. **Keep the learning curve gentle** - Standard library code is easier to understand than hyper internals
3. **Iterate quickly** - No dependency upgrade churn during rapid prototyping
4. **Understand the fundamentals** - Building from scratch teaches HTTP deeply

Once the API surface stabilizes, we'll migrate to `hyper` + `axum` for production-grade performance, HTTP/2 support, and battle-tested reliability.
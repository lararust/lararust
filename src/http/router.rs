use std::sync::Arc;

use crate::http::{
    requests::{Method, Request},
    response::Response,
};

/**
 * Type alias for route handlers that are thread-safe and can be shared across threads.
 */
pub type Handler = Arc<dyn Fn(Request) -> Response + Send + Sync + 'static>;

/**
 * Represents a single route with its HTTP method, path, and handler function.
 */
pub struct Route {
    pub method: Method,
    pub path: String,
    pub handler: Handler,
}

/**
 * HTTP router that registers routes and dispatches incoming requests to handlers.
 *
 * Supports all standard HTTP methods (GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS)
 * with a fluent, chainable API inspired by Laravel's routing.
 *
 * # Example
 * ```
 * let mut router = Router::new();
 *
 * router
 *     .get("/", |_req| Response::html("<h1>Home</h1>"))
 *     .get("/health", |_req| Response::text("OK"))
 *     .post("/users", |req| {
 *         let body = String::from_utf8_lossy(&req.body);
 *         Response::text(&format!("Created: {}", body))
 *     });
 * ```
 */
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    /**
     * Creates a new empty router.
     */
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /**
     * Registers a route with a specific HTTP method, path, and handler.
     *
     * Returns `&mut Self` to enable method chaining.
     */
    pub fn add_route(&mut self, method: Method, path: &str, handler: Handler) -> &mut Self {
        self.routes
            .push(Route { method, path: path.to_string(), handler });
        self
    }

    /**
     * Registers a GET route.
     *
     * # Example
     * ```
     * router.get("/users", |_req| Response::text("List of users"));
     * ```
     */
    pub fn get<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.add_route(Method::GET, path, Arc::new(handler))
    }

    /**
     * Registers a POST route.
     *
     * Typically used for creating resources or submitting form data.
     *
     * # Example
     * ```
     * router.post("/users", |req| {
     *     let body = String::from_utf8_lossy(&req.body);
     *     Response::text(&format!("Created: {}", body))
     * });
     * ```
     */
    pub fn post<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.add_route(Method::POST, path, Arc::new(handler))
    }

    /**
     * Registers a PUT route.
     *
     * Typically used for full resource replacement.
     *
     * # Example
     * ```
     * router.put("/users/123", |req| {
     *     Response::text("User updated")
     * });
     * ```
     */
    pub fn put<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.add_route(Method::PUT, path, Arc::new(handler))
    }

    /**
     * Registers a DELETE route.
     *
     * Used for resource deletion.
     *
     * # Example
     * ```
     * router.delete("/users/123", |_req| {
     *     Response::text("User deleted")
     * });
     * ```
     */
    pub fn delete<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.add_route(Method::DELETE, path, Arc::new(handler))
    }

    /**
     * Registers a PATCH route.
     *
     * Typically used for partial resource updates.
     *
     * # Example
     * ```
     * router.patch("/users/123", |req| {
     *     Response::text("User partially updated")
     * });
     * ```
     */
    pub fn patch<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.add_route(Method::PATCH, path, Arc::new(handler))
    }

    /**
     * Registers an OPTIONS route.
     *
     * Used for CORS preflight requests and API capability discovery.
     * Should return allowed methods and headers without processing the request body.
     *
     * # Example
     * ```
     * router.options("/api/users", |_req| {
     *     Response::new(200, "")
     *         .with_header("Allow", "GET, POST, DELETE")
     *         .with_header("Access-Control-Allow-Methods", "GET, POST, DELETE")
     * });
     * ```
     */
    pub fn options<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.add_route(Method::OPTIONS, path, Arc::new(handler))
    }

    /**
     * Registers a HEAD route.
     *
     * Per HTTP specification, HEAD responses must include the same headers as GET
     * would return, but the server automatically strips the response body.
     * Use HEAD to check resource existence or get metadata without downloading content.
     *
     * # Example
     * ```
     * router.head("/users/123", |_req| {
     *     // Body won't be sent to client anyway
     *     Response::new(200, "")
     * });
     * ```
     */
    pub fn head<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.add_route(Method::HEAD, path, Arc::new(handler))
    }

    /**
     * Dispatches an incoming request to the appropriate handler.
     *
     * Currently performs exact matching on method + path.
     * Returns `Response::not_found()` if no matching route is found.
     *
     * # Future
     * Will support path parameters (e.g., `/users/:id`) in future milestones.
     */
    pub fn dispatch(&self, req: Request) -> Response {
        // Exact match (method + path).
        // TODO: Support path parameters like /users/:id
        for route in &self.routes {
            if route.method == req.method && route.path == req.path {
                return (route.handler)(req);
            }
        }

        Response::not_found()
    }
}

use std::collections::HashMap;

/**
 * HTTP response builder with fluent API for constructing responses.
 *
 * Provides convenience methods for common response types and chainable
 * header manipulation. Automatically sets sensible defaults like Content-Length
 * and Connection headers.
 */
pub struct Response {
    pub body: Vec<u8>,
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
}

impl Response {
    /**
     * Creates a new Response with the given status code and body.
     *
     * Default headers:
     * - `Content-Type: text/plain; charset=utf-8`
     * - `Content-Length: <body_length>`
     * - `Connection: close`
     *
     * # Example
     * ```
     * let response = Response::new(200, "Hello, World!");
     * ```
     */
    pub fn new(status_code: u16, body: impl Into<Vec<u8>>) -> Self {
        let status_text = match status_code {
            200 => "OK",
            201 => "Created",
            204 => "No Content",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "OK",
        }
        .to_string();

        // Capacity should be increased as the project grows
        let mut headers = HashMap::with_capacity(16);

        let body_bytes = body.into();

        headers.insert("Content-Type".to_string(), "text/plain; charset=utf-8".to_string());
        headers.insert("Connection".to_string(), "close".to_string());
        headers.insert("Content-Length".to_string(), body_bytes.len().to_string());

        Response {
            status_code,
            status_text,
            headers,
            body: body_bytes,
        }
    }

    /**
     * Creates a 200 OK response with plain text content.
     *
     * # Example
     * ```
     * router.get("/hello", |_req| Response::text("Hello!"));
     * ```
     */
    pub fn text(text: &str) -> Self {
        Self::new(200, text.as_bytes().to_vec())
    }

    /**
     * Creates a 200 OK response with HTML content.
     *
     * Overrides the default Content-Type to `text/html; charset=utf-8`.
     *
     * # Example
     * ```
     * router.get("/", |_req| Response::html("<h1>Welcome</h1>"));
     * ```
     */
    pub fn html<T: Into<Vec<u8>>>(body: T) -> Self {
        Self::new(200, body).with_header("Content-Type", "text/html; charset=utf-8")
    }

    /// Creates a 404 Not Found response.
    pub fn not_found() -> Self {
        Self::new(404, b"Not Found".to_vec())
    }

    /**
     * Adds or overwrites a header. Returns self for method chaining.
     *
     * # Example
     * ```
     * Response::text("OK")
     *     .with_header("X-Custom", "value")
     *     .with_header("Cache-Control", "no-cache")
     * ```
     */
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers
            .insert(key.to_string(), value.to_string());
        self
    }

    /**
     * Serializes the response into HTTP/1.1 wire format (headers + body).
     *
     * Used for standard HTTP responses (GET, POST, PUT, PATCH, DELETE, etc.).
     */
    pub fn to_http_bytes(&self) -> Vec<u8> {
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str("\r\n");

        let mut bytes = response.into_bytes();
        bytes.extend_from_slice(&self.body);
        bytes
    }

    /**
     * Serializes the response into HTTP/1.1 wire format (headers only, no body).
     *
     * Per HTTP specification, HEAD requests must return the same headers as GET
     * would return, but without the response body. The server automatically calls
     * this method when handling HEAD requests.
     *
     * # Note
     * You typically don't call this directly; the server handles it automatically
     * based on the request method.
     */
    pub fn to_http_bytes_head_only(&self) -> Vec<u8> {
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str("\r\n");

        response.into_bytes()
    }
}

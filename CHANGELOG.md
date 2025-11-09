# Changelog

All notable changes to LaraRust will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-11-09

### Added
- Comprehensive HTTP module documentation (`src/http/README.md`)
- Doc comments for all public HTTP types and methods
- Support for proper HEAD request handling (headers only, no body per HTTP spec)
- Support for proper OPTIONS request handling (CORS preflight and capability discovery)
- `Response::to_http_bytes_head_only()` method for HEAD responses
- Example routes demonstrating all HTTP methods in `src/app.rs`

### Fixed
- **Router method chaining bug**: POST, PUT, PATCH, DELETE, HEAD, and OPTIONS routes were not being registered correctly due to improper method chaining with semicolons
- All route registration methods now properly chain with fluent builder pattern
- HEAD requests now correctly return headers only (no response body) per HTTP/1.1 specification
- Server automatically strips response bodies for HEAD requests

### Changed
- Updated README.md with comprehensive HTTP stack documentation
- Updated ROADMAP.md to reflect handcrafted HTTP stack approach
- Updated CONTRIBUTING.md with current HTTP architecture details
- Improved `sample_router()` to demonstrate proper HTTP method semantics
- Enhanced inline documentation throughout HTTP module

### Documentation
- Added module-level docs to `src/http/mod.rs`
- Added comprehensive function-level docs to:
  - `src/http/response.rs` - Response builder methods
  - `src/http/router.rs` - Route registration methods
  - `src/http/server.rs` - Server initialization and connection handling
  - `src/app.rs` - Sample router and HTTP method semantics

### HTTP Method Semantics
The router now properly implements HTTP specifications:

- **GET, POST, PUT, PATCH, DELETE**: Process request bodies and return full responses
- **HEAD**: Returns the same headers as GET would, but with no response body (automatically handled by server)
- **OPTIONS**: Returns allowed methods and headers for CORS preflight requests and API capability discovery

### Notes
This release focuses on stabilizing the handcrafted HTTP stack before migrating to production dependencies (hyper/axum). The fluent builder pattern for routes is now consistent across all HTTP methods, enabling clean, chainable route definitions.

## [0.1.0] - 2025-11-09

### Added
- Initial handcrafted HTTP stack (Request, Response, Router, Server)
- LaraBlade templating engine
- Basic CLI with `serve` command
- Environment configuration via `.env`
- View caching in production mode

---

[Unreleased]: https://github.com/<org>/lararust/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/<org>/lararust/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/<org>/lararust/releases/tag/v0.1.0
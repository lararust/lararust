<p align="center">
<a href="https://laravel.com" target="_blank"><img src="https://raw.githubusercontent.com/lararust/lararust/refs/heads/main/.assets/lararust-full.png" width="400" alt="Lararust Logo"></a>

</p>

> Laravel’s productivity with Rust’s safety, performance, and native async power.

LaraRust aims to recreate—and eventually surpass—the Laravel developer experience while leaning on Rust’s reliability. It includes a handcrafted HTTP stack (`Request`, `Response`, `Router`, `Server`), Blade-inspired templating, and an opinionated workflow designed for modern cloud workloads.

🚧 **Status:** Pre-MVP (v0.2.0). Follow the roadmap for milestone progress and current focus areas.

## Preview Snapshot (November 2025)

**Current Version: 0.2.0**

- Minimal single-threaded HTTP server backed on `std::net::TcpListener`.
- Ergonomic `Router` with `get/post/put/patch/delete/head/options` helpers with proper HTTP/1.1 semantics.
- Text-based responses out of the box; LaraBlade templates can be rendered inside handlers.
- Proper HEAD request handling (headers only, no body per HTTP spec).
- Proper OPTIONS handling for CORS preflight requests.
- Comprehensive documentation throughout the HTTP module.
- CLI with version information and serve command.

---

## Roadmap & Vision

- **Roadmap:** The complete multi-phase plan—covering CLI, templating, ORM, auth, queues, and beyond—is documented in [`ROADMAP.md`](ROADMAP.md). It includes a Mermaid diagram summarizing dependencies between phases.
- **Philosophy:** Bring Laravel’s DX to Rust without compromising safety or performance. Each phase should ship usable features while laying the foundation for later stages.

---

## Getting Started (Early Preview)

Until the MVP lands, expect frequent breaking changes. The current binary boots the experimental router + HTTP server that live in `src/http` so we can iterate on the Laravel-esque APIs before pulling in heavier dependencies again.

1. Install the latest stable Rust toolchain: `rustup default stable`.
2. Clone the repo and pre-fetch dependencies:
   ```bash
   git clone https://github.com/<org>/lararust.git
   cd lararust
   cargo fetch
   ```
3. Copy `.env.example` if you want to customize the defaults:
   ```bash
   cp .env.example .env
   ```
4. Run the dev server (the CLI and direct binary now share the same boot sequence):
   ```bash
   cargo run
   # or
   cargo run -- serve
   ```
5. Visit http://127.0.0.1:8080 (or whatever `APP_PORT` you set). The sample router already wires the following responses:
   - `/` → renders `resources/views/welcome.larablade.html`
   - `/health` → “OK”
   - `/hello` → “Hello from Lararust Router!”

Use `Ctrl+C` to stop the process; each connection is currently handled on a background thread.

### Routes & Handlers

The router accepts synchronous functions/closures that take a parsed `Request` and return a `Response`. `src/app.rs` centralizes the dev server bootstrap so both `cargo run` and `cargo run -- serve` hit the same code path:

```rust
use crate::{
    http::{response::Response, server::Server},
    prelude::{app_port, load_env, Router},
    view::view,
};

pub fn run_dev_server() {
    load_env();

    let mut router = Router::new();

    router
        .get("/", |_req| {
            let ctx = ctx! {
                "framework" => "LaraRust",
                "admin" => true,
            };

            view("welcome", ctx)
        })
        .get("/health", |_req| Response::text("OK"))
        .get("/hello", |_req| Response::text("Hello from LaraRust Router!"));

    let address = format!("127.0.0.1:{}", app_port());
    Server::new(&address, router).run();
}
```

Routing is currently an exact method + path match. As we iterate we’ll add path params, middleware, and async handlers.

### Configuration

`main.rs` calls `load_env()` before the router boots, so `.env` is read (via `dotenvy`) and simple helpers handle defaults:

| Key        | Default       | Purpose                                                         |
| ---------- | ------------- | --------------------------------------------------------------- |
| `APP_ENV`  | `development` | Controls the banner printed during boot and toggles LaraBlade caching (only on in production). |
| `APP_PORT` | `8080`        | Port for the handcrafted TCP server (must be > 0 and < 65535).  |

Update `.env` or export real environment variables before running `cargo run`. Future milestones will layer in richer config modules.

### Project Layout

```
resources/
└── views/                  # LaraBlade templates (e.g., welcome.larablade.html)
storage/framework/views/    # Cached HTML (auto-created when APP_ENV=production)
src/
├── cli/                    # CLI entry points (stubs while router lives in main.rs)
├── http/                   # Handcrafted Request/Response/Router/Server stack
├── view/                   # LaraBlade compiler, renderer, cache helpers
├── support/                # Environment helpers and future utilities
├── prelude.rs              # Common re-exports
└── main.rs                 # Tokio entry point that boots the router + server
```

Expect this layout to evolve toward the multi-crate structure described in the roadmap.

### LaraBlade Workflow

1. Create templates in `resources/views/<name>.larablade.html` using Blade-like directives (`@if`, `@foreach`, `{{ $var }}`).
2. Call `view("<name>")` from `src/view/mod.rs` inside your handler; it now returns a fully formed `Response` with `text/html` headers so you can hand it straight to the router.
3. Add edge-case data to the temporary context in `src/view/mod.rs` until the formal data layer lands.
4. When `APP_ENV=production`, rendered output is cached under `storage/framework/views` for faster responses; in other environments the compiler re-runs on every request so you can iterate on templates freely.

---

## Contributing

- Start with [`CONTRIBUTING.md`](CONTRIBUTING.md) for workflows, coding standards, testing expectations, and release guidelines.
- Use `.env.example` as your baseline configuration so commands behave consistently in development.
- Read the roadmap issues to pick work aligned with the current phase.
- Join discussions via GitHub Issues/Discussions; major proposals should go through lightweight RFCs.

---

## Code of Conduct

All community spaces follow the [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md), adapted from the Contributor Covenant. Report violations privately to `vinicius.rech.dev@gmail.com`.

---

## License

This project is licensed under the [MIT License](LICENSE). By contributing, you agree that your contributions will be licensed under MIT as well.

---

## Legal

Laravel is a trademark of Taylor Otwell. LaraRust is an independent project that is not affiliated with, endorsed by, or sponsored by the Laravel team.

# LaraRust

> Laravel’s productivity with Rust’s safety, performance, and native async power.

LaraRust aims to recreate—and eventually surpass—the Laravel developer experience while leaning on Rust’s reliability. It provides a CLI-first workflow, expressive routing, Blade-inspired templating, an Eloquent-like ORM, and an opinionated ecosystem designed for modern cloud workloads.

🚧 **Status:** Pre-MVP. Follow the roadmap for milestone progress and current focus areas.

---

## Roadmap & Vision

- **Roadmap:** The complete multi-phase plan—covering CLI, templating, ORM, auth, queues, and beyond—is documented in [`ROADMAP.md`](ROADMAP.md). It includes a Mermaid diagram summarizing dependencies between phases.
- **Philosophy:** Bring Laravel’s DX to Rust without compromising safety or performance. Each phase should ship usable features while laying the foundation for later stages.

---

## Getting Started (Early Preview)

Until the MVP lands, expect frequent breaking changes. The current binary ships with a single `serve` command that boots the Axum-based dev server and serves the static `welcome.html.tera` view.

1. Install the latest stable Rust toolchain: `rustup default stable`.
2. Clone the repo and install dependencies:  
   ```bash
   git clone https://github.com/<org>/lararust.git
   cd lararust
   cargo fetch
   ```
3. Copy the example environment file if you need to tweak defaults:  
   ```bash
   cp .env.example .env
   ```
4. Run the CLI:  
   ```bash
   cargo run -- serve
   ```
5. Visit http://127.0.0.1:8080 (or the `APP_PORT` you configured). `/` renders the welcome template and `/health` returns `OK`.

### Configuration

The runtime reads environment variables via `dotenvy`. Supported keys today:

| Key        | Default       | Purpose                        |
| ---------- | ------------- | ------------------------------ |
| `APP_ENV`  | `development` | Prints the environment banner. |
| `APP_PORT` | `8080`        | Port for the embedded server.  |

Edit `.env` (based on `.env.example`) or set real env vars before running the CLI.

### Project Layout

```
src/
├── cli/            # CLI entry points (`serve` command)
├── http/           # Axum router + server bootstrap
├── resources/      # Temporary view templates
├── support/        # Environment helpers, future utilities
├── prelude.rs      # Common re-exports
└── main.rs         # Tokio entry point calling the CLI
```

Expect this layout to evolve toward the multi-crate structure described in the roadmap.

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

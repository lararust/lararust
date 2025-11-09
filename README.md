# LaraRust

> Laravel’s productivity with Rust’s safety, performance, and native async power.

LaraRust aims to recreate—and eventually surpass—the Laravel developer experience while leaning on Rust’s reliability. It provides a CLI-first workflow, expressive routing, Blade-inspired templating, an Eloquent-like ORM, and an opinionated ecosystem designed for modern cloud workloads.

🚧 **Status:** Pre-MVP. Follow the roadmap for milestone progress and current focus areas.

---

## Roadmap & Vision

- **Roadmap:** The complete multi-phase plan—covering CLI, templating, ORM, auth, queues, and beyond—is documented in [`ROADMAP.md`](ROADMAP.md). It includes a Mermaid diagram summarizing dependencies between phases.
- **Philosophy:** Bring Laravel’s DX to Rust without compromising safety or performance. Each phase should ship usable features while laying the foundation for later stages.

---

## Getting Started (early preview)

Until the MVP lands, expect frequent breaking changes.

1. Install the latest stable Rust toolchain: `rustup default stable`.
2. Clone the repo and run `cargo build`. Use feature flags as phases land.
3. Track CLI commands like `lararust new` or `lararust serve` as they are implemented via the roadmap issues.

As components stabilize, this section will gain concrete installation, configuration, and usage examples.

---

## Contributing

- Start with [`CONTRIBUTING.md`](CONTRIBUTING.md) for workflows, coding standards, testing expectations, and release guidelines.
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

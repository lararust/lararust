# Contributing to LaraRust

Thanks for helping shape LaraRust into a Laravel-class developer experience powered by Rust's safety and performance. The roadmap (`ROADMAP.md`) captures the long-term vision—this guide explains how to collaborate effectively today.

---

## 1. Ground Rules

- **Empathy first:** assume good intent, keep reviews constructive, and document decisions.
- **Security & stability over speed:** prioritize safe APIs and predictable DX, even in early phases.
- **Rust best practices:** embrace ownership, explicit errors, and strong typing.
- **Roadmap alignment:** connect every issue/PR to a roadmap phase or RFC so work ladders up.

---

## 2. Prerequisites & Tooling

| Tool | Purpose |
| ---- | ------- |
| Rust (stable) | Core development (`rustup default stable`). |
| Rust (nightly, optional) | Experiments with proc-macros/optimizations. |
| Cargo tools | `cargo fmt`, `cargo clippy`, `cargo test`. |
| Node.js + npm (optional) | CLI UX prototypes or docs site. |
| Git & GitHub | Workflow management. |

Install via `rustup` and your OS package manager. Keep toolchains updated (`rustup update`) before starting a feature branch.

---

## 3. Repository Structure

Early milestones may live in a single crate, but the roadmap calls for modular crates (`core`, `http`, `view`, `orm`, etc.). As directories appear, keep them cohesive:

- `cli/`: `lararust` binary, commands, scaffolding templates.
- `framework/` or `core/`: service container, config, helpers.
- `http/`: router integration (hyper + axum), middleware, controllers.
- `view/`: LaraBlade _r/runtime, Tera adapters.
- `orm/`: SQLx abstractions, macros, migrations.
- `docs/` & `examples/`: guides + sample apps.

When adding new areas, update the README and `ROADMAP.md` if it affects deliverables or phase sequencing.

---

## 4. Workflow

1. **Open/claim an issue**
   - Label it with the roadmap phase (e.g., `phase:3-orm`) and type (`bug`, `feature`, `docs`).
   - For new ideas, propose an RFC issue summarizing problem, design, alternatives, and roadmap impact.
2. **Create a branch**
   - `feature/<phase>-<short-description>` (e.g., `feature/05-service-container`).
3. **Keep commits focused**
   - Use imperative messages: `Add Auth middleware guard`, `Fix CLI new command prompts`.
4. **Sync often**
   - `git pull --rebase origin main` to avoid drift.

---

## 5. Coding Standards

- Run `cargo fmt`, `cargo clippy --all-targets --all-features`, and `cargo test --all-features`.
- Prefer small, composable functions; document unsafe blocks with rationale.
- Use `anyhow`/`thiserror` for rich errors; avoid panics in library code.
- Write integration tests for CLI commands, HTTP flows, and ORM queries. Snapshot tests for templates are encouraged.
- When touching async code, ensure `.await` points are cancellation-safe and instrumented (tracing spans).
- Add doc comments (`///`) for public APIs and an example when possible.

---

## 6. Pull Requests

Before opening:

- [ ] Issue linked and roadmap phase noted in the PR description.
- [ ] Tests and linting pass locally.
- [ ] Docs/changelog entries added when behavior changes.
- [ ] Screenshots or CLI recordings provided for UX updates.

PR template outline:

```markdown
## Summary
What and why

## Testing
`cargo test -p cli`
`cargo fmt --check`

## Roadmap Phase
Phase 4 — Auth & Session

## Notes
Follow-ups, trade-offs, screenshots, etc.
```

Reviews focus on correctness, DX, and alignment with the roadmap milestones. Expect at least one maintainer approval before merge.

---

## 7. Issue Guidelines

- **Bugs:** include repro steps, expected vs. actual behavior, OS/toolchain, logs, and minimal code snippets.
- **Features:** describe the developer problem, success criteria, and how it maps to the roadmap. Attach mockups or pseudo-code when useful.
- **Docs:** specify audience (beginner/advanced), target file, and key topics to cover.
- **Tests/benchmarks:** highlight metrics to capture (latency, allocations, etc.).

Mark blockers and dependencies explicitly so work can be sequenced across phases.

---

## 8. Security & Responsible Disclosure

Do **not** open public issues for security vulnerabilities. Email the maintainer or use the private security policy (when published) with:

- Affected component and version.
- Reproduction steps.
- Potential impact.
- Suggested remediation if known.

You’ll receive acknowledgment within 48 hours and coordinated disclosure once patched.

---

## 9. Community Expectations

- Follow the [Code of Conduct](CODE_OF_CONDUCT.md). It is based on the Contributor Covenant and enforced across all project spaces.
- Prefer asynchronous communication (GitHub Issues/Discussions). Synchronous chats (Matrix/Discord) are for quick triage, and important decisions must be recorded in issues.
- Mentorship welcome: if you take an issue, guide newcomers by sharing context or breaking down tasks.

---

## 10. Docs & Knowledge Sharing

- Any new concept (helpers, macros, queue workers, etc.) needs at least a README section or dedicated doc under `docs/`.
- Update `ROADMAP.md` when a deliverable is complete or rescheduled, citing PR numbers.
- For large subsystems, create an ADR (architecture decision record) summarizing the decision, status, and consequences.

---

## 11. Release Management

- Use semantic versioning once crates are published.
- Tag releases with `vX.Y.Z` and include highlights, breaking changes, migration notes, and contributor credits.
- For pre-release milestones (Phase 1/2 MVPs), publish release candidates and gather feedback via issue templates.

---

## 12. Getting Help

- **Setup issues:** start with a discussion thread titled `[setup] <question>`.
- **Design questions:** open an RFC issue; label `discussion`.
- **Stuck on a PR:** convert to draft, ping maintainers with a concise summary of the blocker.

Thanks again for contributing! Together we can deliver “Laravel’s productivity on Rust’s robustness.” 💪🦀

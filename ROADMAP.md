The idea is for this roadmap to serve as the framework's **guiding document**, steering development phases, priorities, releases, and technical goals.

*Laravel is a trademark of Taylor Otwell. LaraRust is independent and not affiliated with the Laravel project.*

---

# 🗺️ Complete Roadmap — LaraRust Framework

> **Goal:** recreate the Laravel experience with the performance, safety, and native parallelism of Rust.
> “Laravel's productivity, on top of Rust's robustness.”

---

## 🏁 PHASE 1 — Framework Fundamentals (functional MVP)

### 🎯 Objective

Have the functional core with CLI, embedded server, routing, and a basic view system.

### 📦 Deliverables

* **CLI (`lararust`)**

  * `lararust new <project>` → scaffolds the initial structure
  * `lararust serve` → starts the embedded HTTP server
  * `lararust version`, `lararust help`
* **HTTP Core**

  * Based on `hyper` + `axum`
  * Routes (`get`, `post`, `put`, `delete`)
  * Basic controllers
  * Middlewares (log, CORS, etc.)
* **Configuration**

  * Modular `.env` + `config/`
  * Helpers like `env("APP_PORT")`
* **View Rendering**

  * Support for `Tera`
  * `view("welcome", context!{})` helper
* **Default project structure**

  ```
  routes/web.rs
  resources/views/
  config/
  .env
  ```

---

## ⚙️ PHASE 2 — Blade Engine (custom template engine)

### 🎯 Objective

Build **LaraBlade**, a Blade-inspired template engine with its own syntax and caching.

### 📦 Deliverables

* **Parser and compiler**

  * Directives: `@if`, `@foreach`, `@include`, `@yield`, `@section`, `@extends`
  * Safe expressions: `{{ var }}` and `{!! raw !!}`
* **Rendering**

  * Cache in `storage/framework/views`
  * Automatic change detection (dev mode)
* **Layouts and sections**

  * Support for nested templates (`@extends('layout')`)
* **Hot Reload**

  * Watcher for `.blade.html` files

---

## 🧱 PHASE 3 — ORM / Rust Eloquent

### 🎯 Objective

Provide a fluent and safe ORM with an Eloquent-inspired syntax.

### 📦 Deliverables

* **SQLx abstraction layer**

  * Support for PostgreSQL and MySQL
* **Model Trait**

  ```rust
  #[derive(Model)]
  pub struct User {
      pub id: i32,
      pub name: String,
  }
  ```
* **Query Builder**

  ```rust
  User::where("age", ">", 18)
      .order_by("created_at", "desc")
      .limit(10)
      .get();
  ```
* **Migrations**

  * `lararust migrate`, `lararust migrate:rollback`
* **Seeders and Factories**

  * `lararust db:seed`

---

## 🔐 PHASE 4 — Authentication and Session

### 🎯 Objective

Deliver the integrated authentication system and persistent sessions.

### 📦 Deliverables

* **Authentication middleware**

  * `auth`, `guest`
* **Session Manager**

  * Cookie + Redis
* **LoginController / RegisterController**

  * `lararust make:auth` scaffolds default routes and views
* **Hashing and Encryption**

  * Native Argon2 / bcrypt
* **CSRF and Tokens**

  * Automatic `@csrf` in forms

---

## 🧩 PHASE 5 — Middleware and Service Container

### 🎯 Objective

Introduce dependency injection, providers, and configurable middlewares.

### 📦 Deliverables

* **Service Container**

  * Dependency registration
  * Automatic injection in controllers
* **Service Providers**

  * `AppServiceProvider`, `AuthServiceProvider`, etc.
* **Custom middlewares**

  * `lararust make:middleware`

---

## ⚡ PHASE 6 — Jobs, Queues, and Events

### 🎯 Objective

Add background jobs and the event/queue system.

### 📦 Deliverables

* **Job system**

  * `lararust queue:work`
  * `#[derive(Job)]`
* **Redis and RabbitMQ support**

  * `queue` module
* **Events and listeners**

  * `Event::dispatch()`
  * `#[derive(Event)]`
* **Schedule**

  * `lararust schedule:run`
  * Internal cron with `tokio::time`

---

## 🧰 PHASE 7 — Full CLI and ecosystem

### 🎯 Objective

Turn the CLI into a true `artisan`.

### 📦 Deliverables

* `lararust make:model User`
* `lararust make:controller UserController`
* `lararust make:migration create_users_table`
* `lararust make:middleware`
* `lararust test`
* **Modular project generator**

  * `lararust new api`, `lararust new web`

---

## 🌐 PHASE 8 — API & JSON Layer

### 🎯 Objective

Make it easy to build REST APIs and hybrid (SSR + JSON) APIs.

### 📦 Deliverables

* **Response Helpers**

  * `json!()`, `success!()`, `error!()`
* **Request Validation**

  * `#[derive(Validate)]`
* **FormRequest**

  * `lararust make:request`
* **Pagination**

  * `User::paginate(10)`

---

## 🧠 PHASE 9 — Macros, Helpers, and Facades

### 🎯 Objective

Bring the syntactic sugar that makes Laravel a joy to use.

### 📦 Deliverables

* **Macros**

  * `String::macro("slugify", |s| s.to_lowercase())`
* **Global Helpers**

  * `route("home")`, `config("app.name")`, `now()`
* **Facades**

  * `View::make("home")`, `DB::query()`, `Log::info()`

---

## 🧩 PHASE 10 — Modularization by Crates

### 🎯 Objective

Split LaraRust into independent yet integrated packages.

### 📦 Structure

```
lararust/
├── core/
├── http/
├── view/
├── orm/
├── cli/
├── support/
├── queue/
└── router/
```

Each crate gets published on crates.io, while the main `lararust` crate re-exports everything.

---

## 🔥 PHASE 11 — Ecosystem and Extensions

### 🎯 Objective

Expand LaraRust into a platform.

### 📦 Deliverables

* **LaraRust Breeze** — starter kit (login, register, views)
* **LaraRust Nova** — admin panel
* **LaraRust Sail** — integrated Docker environment
* **LaraRust Sanctum / Passport** — API authentication with JWT tokens
* **LaraRust Octane** — serverless mode + HTTP/2
* **LaraRust Forge** — automated deployment tool
* **LaraRust Scheduler** — cron/async job manager

---

## 🧠 PHASE 12 — Testing and Quality

### 🎯 Objective

Guarantee reliability and top-tier DX.

### 📦 Deliverables

* `lararust test`
* Route, controller, and model testing
* `assert_view_contains!()`, `assert_json!()`
* Integrated coverage
* Benchmark suite against Laravel, Axum, and Rocket

---

## 🏗️ PHASE 13 — Full-stack Web Framework

### 🎯 Objective

Unify SSR + SPA.

### 📦 Deliverables

* **Inertia-like Integration**

  * `lararust inertia vue`
* **Integrated WebSocket**

  * `lararust ws`
* **Assets Pipeline**

  * optional support for Vite/Tailwind
* **Livewire Rust**

  * WebSocket-powered reactivity (stateful components)

---

## 🌍 PHASE 14 — Deploy, Monitoring, and Cloud

### 🎯 Objective

Simplify deployment and observability.

### 📦 Deliverables

* `lararust deploy aws|render|fly.io`
* `lararust logs --tail`
* `lararust metrics`
* Prometheus monitoring
* Automated container-based deployment

---

## 🏁 Final Result

| Area            | Laravel             | LaraRust                           |
| --------------- | ------------------- | ---------------------------------- |
| CLI             | Artisan             | LaraCLI                            |
| Template Engine | Blade               | LaraBlade                          |
| ORM             | Eloquent            | RustORM                            |
| Routes          | Router              | Router (Axum)                      |
| Views           | Blade               | Blade-like Engine                  |
| Auth            | Auth Guard          | Auth Manager                       |
| Queue           | Redis / RabbitMQ    | Async Jobs                         |
| Config          | `.env`, `config/`   | `.env`, TOML                       |
| Providers       | Service Providers   | DI + Providers                     |
| Facades         | Yes                 | Yes                                |
| Ecosystem       | Forge, Nova, Breeze | Forge-Rust, Nova-Rust, Breeze-Rust |

---

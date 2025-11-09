pub mod cache;
pub mod helpers;
pub mod larablade;
pub mod renderer;

use crate::http::response::Response;
use crate::support::env::app_env;
use cache::{cache_view, get_cached};
use larablade::compile_larablade;
use renderer::render_larablade;
use std::fs;
use tera::Context;

/// Renders a LaraBlade template and wraps it in an HTTP `Response`.
pub fn view(template: &str, context: Context) -> Response {
    let rendered = render_view(template, context);
    Response::html(rendered.into_bytes())
}

/// Renders a LaraBlade template and returns the HTML string.
pub fn render_view(template: &str, context: Context) -> String {
    let use_cache = app_env().eq_ignore_ascii_case("production");

    if use_cache {
        if let Some(cached) = get_cached(template) {
            println!("♻️ Serving cached view: {}", template);
            return cached;
        }
    }

    let path = format!("resources/views/{}.larablade.html", template);
    let raw = fs::read_to_string(&path).unwrap_or_else(|error| {
        eprintln!("Error reading view '{}': {}", template, error);
        String::from("<h1>Error loading view</h1>")
    });

    let compiled = compile_larablade(&raw);
    let rendered = render_larablade(&compiled, &context);

    if use_cache {
        cache_view(template, &rendered);
    }

    rendered
}

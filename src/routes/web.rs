use crate::view::view;
use tera::Context;

pub async fn home() -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("name", "Jhon Doe");
    ctx.insert("admin", &true);
    Html(view("welcome", ctx))
}

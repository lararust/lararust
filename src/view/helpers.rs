/// Creates a Tera Context from key-value pairs using JSON-like syntax.
///
/// Example:
/// ```
/// let ctx = ctx! {
///     "name" => "Vinícius",
///     "admin" => true,
/// };
/// ```
#[macro_export]
macro_rules! ctx {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        let mut context = tera::Context::new();
        let data = serde_json::json!({ $( $key: $value ),* });
        if let serde_json::Value::Object(map) = data {
            for (k, v) in map {
                context.insert(k, &v);
            }
        }
        context
    }};
}

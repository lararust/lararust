use serde::Serialize;
use tera::Context;

/// Trait that converts any serializable struct into a Tera Context.
pub trait IntoContext {
    fn into_context(&self) -> Context;
}

impl<T> IntoContext for T
where
    T: Serialize,
{
    fn into_context(&self) -> Context {
        let mut context = Context::new();
        let value = serde_json::to_value(self).unwrap();
        if let serde_json::Value::Object(map) = value {
            for (k, v) in map {
                context.insert(k, &v);
            }
        }
        context
    }
}

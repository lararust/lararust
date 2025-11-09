/**
 * LaraRust framework version.
 * Automatically synced with Cargo.toml version.
 */
#[allow(dead_code)]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use crate::http::router::*;
pub use crate::support::env::*;

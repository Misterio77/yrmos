pub static VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod common;
pub mod routes;
pub mod schema;

pub use common::{config, errors, layouts, state, style};
pub use errors::AppError;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod routes;
pub mod schema;

pub mod state;
pub use state::AppState;
pub mod config;
pub use config::AppConfig;

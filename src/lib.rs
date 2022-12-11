pub static VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod errors;
pub use errors::AppError;

pub mod routes;
pub mod schema;

pub mod layouts;
pub mod style;
pub use style::StyleSheet;

pub mod state;
pub use state::AppState;
pub mod config;
pub use config::AppConfig;

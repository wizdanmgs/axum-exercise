use crate::state::SharedState;
use axum::{Router, routing::get};

mod health;
mod root;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(root::root))
        .route("/health", get(health::health))
}

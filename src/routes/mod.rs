use crate::state::SharedState;
use axum::{
    Router,
    routing::{get, post},
};

mod echo;
mod greet;
mod health;
mod root;
mod users;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(root::root))
        .route("/health", get(health::health))
        .route("/greet", get(greet::greet))
        .route("/users/{id}", get(users::get_user))
        .route("/echo", post(echo::echo))
}

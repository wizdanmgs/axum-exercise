use axum::Router;
use tower_http::services::ServeDir;

use crate::{middleware, routes, state};

pub fn create_app() -> Router {
    let shared_state = state::create_state();

    let router = Router::new()
        .merge(routes::routes())
        .nest_service("/static", ServeDir::new("static"))
        .with_state(shared_state);

    middleware::apply(router)
}

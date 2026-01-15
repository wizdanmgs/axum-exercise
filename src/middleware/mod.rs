use axum::Router;
use tower_http::trace::TraceLayer;

pub fn apply(router: Router) -> Router {
    router.layer(TraceLayer::new_for_http())
}

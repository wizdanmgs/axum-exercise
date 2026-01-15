use axum::{extract::State, response::IntoResponse};

use crate::state::SharedState;

pub async fn root(State(state): State<SharedState>) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    state.visits += 1;

    format!("Visits: {}", state.visits)
}

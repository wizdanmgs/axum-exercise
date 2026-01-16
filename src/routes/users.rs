use axum::{extract::Path, response::IntoResponse};

pub async fn get_user(Path(user_id): Path<u32>) -> impl IntoResponse {
    format!("User id: {}", user_id)
}

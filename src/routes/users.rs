use axum::{extract::Path, response::IntoResponse};

use crate::error::ApiError;

pub async fn get_user(Path(user_id): Path<u32>) -> Result<impl IntoResponse, ApiError> {
    if user_id == 0 {
        return Err(ApiError::BadRequest(
            "user_id must be greater than 0".into(),
        ));
    }

    Ok(format!("User id: {}", user_id))
}

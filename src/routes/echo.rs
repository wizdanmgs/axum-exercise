use axum::{extract::Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::error::ApiError;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct EchoPayload {
    #[validate(length(min = 3, message = "message must be at least 3 characters"))]
    pub message: String,
}

pub async fn echo(Json(payload): Json<EchoPayload>) -> Result<impl IntoResponse, ApiError> {
    payload.validate()?;
    Ok(Json(payload))
}

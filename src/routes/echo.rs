use axum::{extract::Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EchoPayload {
    pub message: String,
}

pub async fn echo(Json(payload): Json<EchoPayload>) -> impl IntoResponse {
    Json(payload)
}

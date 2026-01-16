use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("validation error")]
    Validation(#[from] ValidationErrors),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("internal server error")]
    Internal,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    details: Option<serde_json::Value>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Validation(errs) => {
                let body = ErrorResponse {
                    error: "validation error".into(),
                    details: Some(serde_json::to_value(errs).unwrap()),
                };

                (StatusCode::BAD_REQUEST, Json(body)).into_response()
            }

            ApiError::BadRequest(msg) => {
                let body = ErrorResponse {
                    error: msg,
                    details: None,
                };

                (StatusCode::BAD_REQUEST, Json(body)).into_response()
            }

            ApiError::Internal => {
                let body = ErrorResponse {
                    error: "internal server error".into(),
                    details: None,
                };

                (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
            }
        }
    }
}

use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;
use validator::Validate;

use crate::error::ApiError;

#[derive(Deserialize, Debug, Validate)]
pub struct GreetQuery {
    #[validate(length(min = 3, message = "name too short"))]
    pub name: Option<String>,
}

pub async fn greet(Query(query): Query<GreetQuery>) -> Result<impl IntoResponse, ApiError> {
    query.validate()?;

    let name = query.name.unwrap_or_else(|| "World".to_string());
    Ok(format!("Hello, {}!", name))
}

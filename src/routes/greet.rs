use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GreetQuery {
    pub name: Option<String>,
}

pub async fn greet(Query(query): Query<GreetQuery>) -> impl IntoResponse {
    let name = query.name.unwrap_or_else(|| "World".to_string());
    format!("Hello, {}!", name)
}

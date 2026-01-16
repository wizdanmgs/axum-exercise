use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

/// Send a GET Request
pub async fn get(app: &axum::Router, uri: &str) -> Response<Body> {
    app.clone()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap()
}

/// Send a POST request with JSON body
pub async fn post_json(app: &axum::Router, uri: &str, body: serde_json::Value) -> Response<Body> {
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("Content-Type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap()
}

/// Assert HTTP status code
pub fn assert_status(response: &Response<Body>, expected: StatusCode) {
    assert_eq!(response.status(), expected);
}

/// Read body as bytes
pub async fn body_bytes(response: Response<Body>) -> bytes::Bytes {
    response.into_body().collect().await.unwrap().to_bytes()
}

/// Read body as JSON
pub async fn body_json(response: Response<Body>) -> serde_json::Value {
    serde_json::from_slice(&body_bytes(response).await).unwrap()
}

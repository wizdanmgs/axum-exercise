use axum::{
    body::Body,
    http::{Request, StatusCode, response},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

use axum_exercise::app;

#[tokio::test]
async fn health_check_works() {
    let app = app::create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn visits_counter_increments() {
    let app = app::create_app();

    let res1 = app
        .clone()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body1 = res1.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body1[..], b"Visits: 1");

    let res2 = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body2 = res2.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body2[..], b"Visits: 2");
}

#[tokio::test]
async fn greet_validation_fails() {
    let app = app::create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/greet?name=a")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn user_id_validation_fails() {
    let app = app::create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/users/0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn echo_success() {
    let app = app::create_app();

    let payload = serde_json::json!({
        "message": "test"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/echo")
                .header("Content-Type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["message"], "test");
}

#[tokio::test]
async fn echo_validation_fails() {
    let app = app::create_app();

    let payload = serde_json::json!({
        "message": "t"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/echo")
                .header("Content-Type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

mod helpers;

use axum::http::StatusCode;
use axum_exercise::app;
use helpers::*;

#[tokio::test]
async fn health_check_works() {
    let app = app::create_app();

    let res = get(&app, "/health").await;

    assert_status(&res, StatusCode::OK);
}

#[tokio::test]
async fn visits_counter_increments() {
    let app = app::create_app();

    let res1 = get(&app, "/").await;
    let body1 = body_bytes(res1).await;
    assert_eq!(&body1[..], b"Visits: 1");

    let res2 = get(&app, "/").await;
    let body2 = body_bytes(res2).await;
    assert_eq!(&body2[..], b"Visits: 2");
}

#[tokio::test]
async fn greet_validation_fails() {
    let app = app::create_app();

    let res = get(&app, "/greet?name=a").await;

    assert_status(&res, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn user_id_validation_fails() {
    let app = app::create_app();

    let res = get(&app, "/users/0").await;

    assert_status(&res, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn echo_success() {
    let app = app::create_app();

    let res = post_json(&app, "/echo", serde_json::json!({"message": "test"})).await;

    assert_status(&res, StatusCode::OK);

    let json = body_json(res).await;

    assert_eq!(json["message"], "test");
}

#[tokio::test]
async fn echo_validation_fails() {
    let app = app::create_app();

    let res = post_json(&app, "/echo", serde_json::json!({"message": "t"})).await;

    assert_status(&res, StatusCode::BAD_REQUEST);
}

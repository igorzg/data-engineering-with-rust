use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use serde_json::{json, Value};
use tower::util::ServiceExt;

use challange_week3::create_router;

async fn get(uri: &str) -> (StatusCode, String) {
    let response = create_router()
        .oneshot(
            Request::builder()
                .uri(uri)
                .body(Body::empty())
                .expect("valid request"),
        )
        .await
        .expect("router handled request");
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body is readable");
    (status, String::from_utf8(body.to_vec()).expect("body is utf8"))
}

#[tokio::test]
async fn test_root_route() {
    let (status, body) = get("/").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Greedy Coin Change Machine"));
    assert!(body.contains("/change/:dollars/:cents"));
}

#[tokio::test]
async fn test_change_route_returns_coins() {
    let (status, body) = get("/change/1/26").await;
    assert_eq!(status, StatusCode::OK);
    let json: Value = serde_json::from_str(&body).expect("valid json");
    assert_eq!(json, json!({ "dollars": 1, "cents": 26, "change": [25, 25, 25, 25, 25, 1] }));
}

#[tokio::test]
async fn test_change_route_exact_quarters() {
    let (status, body) = get("/change/2/00").await;
    assert_eq!(status, StatusCode::OK);
    let json: Value = serde_json::from_str(&body).expect("valid json");
    assert_eq!(json, json!({ "dollars": 2, "cents": 0, "change": [25, 25, 25, 25, 25, 25, 25, 25] }));
}

#[tokio::test]
async fn test_change_route_zero_amount() {
    let (status, body) = get("/change/0/0").await;
    assert_eq!(status, StatusCode::OK);
    let json: Value = serde_json::from_str(&body).expect("valid json");
    assert_eq!(json, json!({ "dollars": 0, "cents": 0, "change": [] }));
}

#[tokio::test]
async fn test_change_route_mixed_coins() {
    let (status, body) = get("/change/0/84").await;
    assert_eq!(status, StatusCode::OK);
    let json: Value = serde_json::from_str(&body).expect("valid json");
    assert_eq!(
        json,
        json!({ "dollars": 0, "cents": 84, "change": [25, 25, 25, 5, 1, 1, 1, 1] })
    );
}

#[tokio::test]
async fn test_change_route_rejects_non_numeric_path() {
    let (status, _) = get("/change/abc/def").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_unknown_route_returns_404() {
    let (status, _) = get("/nope").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

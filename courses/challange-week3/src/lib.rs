use axum::extract::Path;
use axum::{routing::get, Json, Router};
use serde_json::json;

pub fn greedy_coin_change(amount: u32) -> Vec<u32> {
    let coins = [25, 10, 5, 1];
    let mut change = Vec::new();
    let mut remaining = amount;

    for coin in coins {
        while remaining >= coin {
            remaining -= coin;
            change.push(coin);
        }
    }

    change
}

pub async fn root() -> &'static str {
    "
    Greedy Coin Change Machine

    ** Primary Route: **
    /change/:dollars/:cents
    "
}

pub async fn change(Path((dollars, cents)): Path<(u32, u32)>) -> impl axum::response::IntoResponse {
    let amount = dollars * 100 + cents;
    let change = greedy_coin_change(amount);
    let json = json!({
        "dollars": dollars,
        "cents": cents,
        "change": change
    });
    Json(json)
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/change/{dollars}/{cents}", get(change))
}

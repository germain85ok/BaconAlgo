use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub entry_price: f64,
    pub current_price: f64,
    pub pnl: f64,
    pub pnl_percent: f64,
    pub side: String,
    pub opened_at: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub total_value: f64,
    pub cash_balance: f64,
    pub positions: Vec<Position>,
    pub daily_pnl: f64,
    pub total_pnl: f64,
    pub timestamp: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub symbol: String,
    pub side: String,
    pub quantity: f64,
    pub order_type: String,
    pub limit_price: Option<f64>,
    pub stop_price: Option<f64>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order_id: String,
    pub status: String,
    pub filled_quantity: f64,
    pub average_price: f64,
    pub timestamp: i64,
}

pub async fn get_portfolio() -> Result<Json<Portfolio>, StatusCode> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let positions = vec![
        Position {
            symbol: "BTCUSD".to_string(),
            quantity: 0.5,
            entry_price: 44000.0,
            current_price: 45000.0,
            pnl: 500.0,
            pnl_percent: 2.27,
            side: "LONG".to_string(),
            opened_at: now - 86400,
        },
    ];

    let portfolio = Portfolio {
        total_value: 50000.0,
        cash_balance: 27500.0,
        positions,
        daily_pnl: 750.0,
        total_pnl: 2500.0,
        timestamp: now,
    };

    Ok(Json(portfolio))
}

pub async fn create_order(
    Json(order): Json<OrderRequest>,
) -> Result<Json<OrderResponse>, StatusCode> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let response = OrderResponse {
        order_id: format!("ORD-{}", now),
        status: "PENDING".to_string(),
        filled_quantity: 0.0,
        average_price: order.limit_price.unwrap_or(0.0),
        timestamp: now,
    };

    Ok(Json(response))
}

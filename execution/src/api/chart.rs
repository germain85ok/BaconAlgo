use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use crate::market::ProviderManager;

#[derive(Debug, Deserialize)]
pub struct ChartParams {
    #[serde(default = "default_timeframe")]
    timeframe: String,
    #[serde(default = "default_limit")]
    limit: usize,
}

fn default_timeframe() -> String {
    "15".to_string()
}

fn default_limit() -> usize {
    500
}

#[derive(Debug, Serialize)]
pub struct ChartResponse {
    symbol: String,
    timeframe: String,
    candles: Vec<CandleData>,
    count: usize,
}

#[derive(Debug, Serialize)]
pub struct CandleData {
    time: i64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

/// GET /api/chart/{symbol}?timeframe=15&limit=500
/// Returns OHLCV candle data for charting
pub async fn get_chart_data(
    Path(symbol): Path<String>,
    Query(params): Query<ChartParams>,
    State(provider_manager): State<Arc<ProviderManager>>,
) -> impl IntoResponse {
    tracing::info!(
        "Chart data requested for {} with timeframe {} and limit {}",
        symbol,
        params.timeframe,
        params.limit
    );

    match provider_manager.get_candles(&symbol, &params.timeframe, params.limit).await {
        Ok(candles) => {
            let chart_candles: Vec<CandleData> = candles
                .into_iter()
                .map(|c| CandleData {
                    time: c.timestamp / 1000, // Convert to seconds for TradingView
                    open: c.open,
                    high: c.high,
                    low: c.low,
                    close: c.close,
                    volume: c.volume,
                })
                .collect();

            let response = ChartResponse {
                symbol: symbol.clone(),
                timeframe: params.timeframe.clone(),
                count: chart_candles.len(),
                candles: chart_candles,
            };

            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            tracing::error!("Failed to fetch chart data for {}: {}", symbol, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ChartResponse {
                    symbol,
                    timeframe: params.timeframe,
                    candles: Vec::new(),
                    count: 0,
                }),
            )
        }
    }
}

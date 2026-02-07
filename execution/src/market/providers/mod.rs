pub mod yahoo;
pub mod binance;
pub mod alphavantage;
pub mod finnhub;
pub mod twelvedata;
pub mod coingecko;
pub mod polygon;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// OHLCV candle data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub symbol: String,
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// Real-time quote data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub symbol: String,
    pub price: f64,
    pub timestamp: i64,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub volume: Option<f64>,
}

/// Provider error type
#[derive(Debug)]
pub enum ProviderError {
    RateLimitExceeded,
    InvalidApiKey,
    SymbolNotFound,
    NetworkError(String),
    ParseError(String),
    Other(String),
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            ProviderError::InvalidApiKey => write!(f, "Invalid API key"),
            ProviderError::SymbolNotFound => write!(f, "Symbol not found"),
            ProviderError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ProviderError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ProviderError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl Error for ProviderError {}

/// Trait that all market data providers must implement
#[async_trait]
pub trait MarketDataProvider: Send + Sync {
    /// Get the provider name
    fn name(&self) -> &str;
    
    /// Check if the provider is available (API key configured, etc.)
    fn is_available(&self) -> bool;
    
    /// Get real-time quote for a symbol
    async fn get_quote(&self, symbol: &str) -> Result<Quote, ProviderError>;
    
    /// Get historical candles
    /// interval: time interval in minutes (1, 5, 15, 60, etc.) or "D" for daily
    /// limit: number of candles to retrieve
    async fn get_candles(
        &self,
        symbol: &str,
        interval: &str,
        limit: usize,
    ) -> Result<Vec<Candle>, ProviderError>;
    
    /// Get bulk quotes for multiple symbols (if supported)
    async fn get_bulk_quotes(&self, symbols: &[String]) -> Result<Vec<Quote>, ProviderError> {
        // Default implementation: fetch quotes one by one
        let mut quotes = Vec::new();
        for symbol in symbols {
            match self.get_quote(symbol).await {
                Ok(quote) => quotes.push(quote),
                Err(e) => {
                    tracing::warn!("Failed to get quote for {}: {}", symbol, e);
                }
            }
        }
        Ok(quotes)
    }
}

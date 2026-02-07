use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use super::{MarketDataProvider, Candle, Quote, ProviderError};

/// Binance provider - PRIMARY for cryptocurrency data
pub struct BinanceProvider {
    client: reqwest::Client,
    api_key: Option<String>,
    api_secret: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BinanceKline {
    #[serde(rename = "0")]
    open_time: i64,
    #[serde(rename = "1")]
    open: String,
    #[serde(rename = "2")]
    high: String,
    #[serde(rename = "3")]
    low: String,
    #[serde(rename = "4")]
    close: String,
    #[serde(rename = "5")]
    volume: String,
}

#[derive(Debug, Deserialize)]
struct BinanceTicker {
    symbol: String,
    #[serde(rename = "lastPrice")]
    last_price: String,
    #[serde(rename = "bidPrice")]
    bid_price: Option<String>,
    #[serde(rename = "askPrice")]
    ask_price: Option<String>,
    volume: Option<String>,
}

impl BinanceProvider {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(),
            api_key,
            api_secret,
        }
    }

    /// Convert interval to Binance interval format
    fn convert_interval(interval: &str) -> String {
        match interval {
            "1" => "1m".to_string(),
            "5" => "5m".to_string(),
            "15" => "15m".to_string(),
            "30" => "30m".to_string(),
            "60" => "1h".to_string(),
            "240" => "4h".to_string(),
            "D" | "1D" => "1d".to_string(),
            "W" | "1W" => "1w".to_string(),
            _ => format!("{}m", interval),
        }
    }
}

#[async_trait]
impl MarketDataProvider for BinanceProvider {
    fn name(&self) -> &str {
        "Binance"
    }

    fn is_available(&self) -> bool {
        // Binance API can work without API key for public endpoints
        true
    }

    async fn get_quote(&self, symbol: &str) -> Result<Quote, ProviderError> {
        let url = format!(
            "https://api.binance.com/api/v3/ticker/24hr?symbol={}",
            symbol
        );

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            if response.status().as_u16() == 429 {
                return Err(ProviderError::RateLimitExceeded);
            }
            return Err(ProviderError::NetworkError(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let ticker: BinanceTicker = response
            .json()
            .await
            .map_err(|e| ProviderError::ParseError(e.to_string()))?;

        Ok(Quote {
            symbol: symbol.to_string(),
            price: ticker.last_price.parse().unwrap_or(0.0),
            timestamp: chrono::Utc::now().timestamp_millis(),
            bid: ticker.bid_price.and_then(|b| b.parse().ok()),
            ask: ticker.ask_price.and_then(|a| a.parse().ok()),
            volume: ticker.volume.and_then(|v| v.parse().ok()),
        })
    }

    async fn get_candles(
        &self,
        symbol: &str,
        interval: &str,
        limit: usize,
    ) -> Result<Vec<Candle>, ProviderError> {
        let binance_interval = Self::convert_interval(interval);
        
        // Binance max limit is 1000
        let limit = std::cmp::min(limit, 1000);

        let url = format!(
            "https://api.binance.com/api/v3/klines?symbol={}&interval={}&limit={}",
            symbol, binance_interval, limit
        );

        tracing::debug!("Fetching Binance candles: {}", url);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            if response.status().as_u16() == 429 {
                return Err(ProviderError::RateLimitExceeded);
            }
            return Err(ProviderError::NetworkError(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let klines: Vec<serde_json::Value> = response
            .json()
            .await
            .map_err(|e| ProviderError::ParseError(e.to_string()))?;

        let mut candles = Vec::new();

        for kline in klines {
            if let serde_json::Value::Array(data) = kline {
                if data.len() >= 6 {
                    let open_time = data[0].as_i64().unwrap_or(0);
                    let open = data[1].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                    let high = data[2].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                    let low = data[3].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                    let close = data[4].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                    let volume = data[5].as_str().unwrap_or("0").parse().unwrap_or(0.0);

                    candles.push(Candle {
                        symbol: symbol.to_string(),
                        timestamp: open_time,
                        open,
                        high,
                        low,
                        close,
                        volume,
                    });
                }
            }
        }

        Ok(candles)
    }

    async fn get_bulk_quotes(&self, symbols: &[String]) -> Result<Vec<Quote>, ProviderError> {
        // Binance supports bulk tickers
        if symbols.is_empty() {
            return Ok(Vec::new());
        }

        let url = "https://api.binance.com/api/v3/ticker/24hr";

        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            if response.status().as_u16() == 429 {
                return Err(ProviderError::RateLimitExceeded);
            }
            return Err(ProviderError::NetworkError(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let tickers: Vec<BinanceTicker> = response
            .json()
            .await
            .map_err(|e| ProviderError::ParseError(e.to_string()))?;

        let symbol_set: std::collections::HashSet<String> = symbols.iter().cloned().collect();

        let quotes: Vec<Quote> = tickers
            .into_iter()
            .filter(|t| symbol_set.contains(&t.symbol))
            .map(|ticker| Quote {
                symbol: ticker.symbol,
                price: ticker.last_price.parse().unwrap_or(0.0),
                timestamp: chrono::Utc::now().timestamp_millis(),
                bid: ticker.bid_price.and_then(|b| b.parse().ok()),
                ask: ticker.ask_price.and_then(|a| a.parse().ok()),
                volume: ticker.volume.and_then(|v| v.parse().ok()),
            })
            .collect();

        Ok(quotes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_binance_get_quote() {
        let provider = BinanceProvider::new(None, None);
        let quote = provider.get_quote("BTCUSDT").await;
        
        // Only test if we have network access
        if let Ok(q) = quote {
            assert_eq!(q.symbol, "BTCUSDT");
            assert!(q.price > 0.0);
        }
    }

    #[tokio::test]
    async fn test_binance_get_candles() {
        let provider = BinanceProvider::new(None, None);
        let candles = provider.get_candles("BTCUSDT", "15", 50).await;
        
        // Only test if we have network access
        if let Ok(c) = candles {
            assert!(!c.is_empty());
            assert!(c[0].close > 0.0);
        }
    }
}

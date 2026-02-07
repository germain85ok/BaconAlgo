use async_trait::async_trait;
use super::{MarketDataProvider, Candle, Quote, ProviderError};

/// Alpha Vantage provider with key rotation
pub struct AlphaVantageProvider {
    client: reqwest::Client,
    api_keys: Vec<String>,
    current_key_index: std::sync::atomic::AtomicUsize,
}

impl AlphaVantageProvider {
    pub fn new(keys: Vec<String>) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(15))
                .build()
                .unwrap(),
            api_keys: keys.into_iter().filter(|k| !k.is_empty()).collect(),
            current_key_index: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    fn get_next_key(&self) -> Option<&str> {
        if self.api_keys.is_empty() {
            return None;
        }
        let index = self.current_key_index.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % self.api_keys.len();
        Some(&self.api_keys[index])
    }
}

#[async_trait]
impl MarketDataProvider for AlphaVantageProvider {
    fn name(&self) -> &str {
        "Alpha Vantage"
    }

    fn is_available(&self) -> bool {
        !self.api_keys.is_empty()
    }

    async fn get_quote(&self, symbol: &str) -> Result<Quote, ProviderError> {
        let api_key = self.get_next_key().ok_or(ProviderError::InvalidApiKey)?;
        
        let url = format!(
            "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
            symbol, api_key
        );

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ProviderError::ParseError(e.to_string()))?;

        if let Some(note) = data.get("Note") {
            return Err(ProviderError::RateLimitExceeded);
        }

        let global_quote = data.get("Global Quote")
            .ok_or_else(|| ProviderError::SymbolNotFound)?;

        let price: f64 = global_quote.get("05. price")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| ProviderError::ParseError("No price data".to_string()))?;

        Ok(Quote {
            symbol: symbol.to_string(),
            price,
            timestamp: chrono::Utc::now().timestamp_millis(),
            bid: None,
            ask: None,
            volume: None,
        })
    }

    async fn get_candles(
        &self,
        symbol: &str,
        _interval: &str,
        _limit: usize,
    ) -> Result<Vec<Candle>, ProviderError> {
        // Simplified implementation - Alpha Vantage rate limits are strict
        tracing::warn!("Alpha Vantage candles not fully implemented - returning empty");
        Ok(Vec::new())
    }
}

use async_trait::async_trait;
use super::{MarketDataProvider, Candle, Quote, ProviderError};

/// Finnhub provider for real-time quotes
pub struct FinnhubProvider {
    client: reqwest::Client,
    api_key: Option<String>,
}

impl FinnhubProvider {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(),
            api_key,
        }
    }
}

#[async_trait]
impl MarketDataProvider for FinnhubProvider {
    fn name(&self) -> &str {
        "Finnhub"
    }

    fn is_available(&self) -> bool {
        self.api_key.is_some()
    }

    async fn get_quote(&self, symbol: &str) -> Result<Quote, ProviderError> {
        let api_key = self.api_key.as_ref().ok_or(ProviderError::InvalidApiKey)?;
        
        let url = format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token={}",
            symbol, api_key
        );

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        if response.status().as_u16() == 429 {
            return Err(ProviderError::RateLimitExceeded);
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ProviderError::ParseError(e.to_string()))?;

        let price = data.get("c")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| ProviderError::SymbolNotFound)?;

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
        _symbol: &str,
        _interval: &str,
        _limit: usize,
    ) -> Result<Vec<Candle>, ProviderError> {
        // Simplified - Finnhub candles require more complex implementation
        tracing::warn!("Finnhub candles not fully implemented - returning empty");
        Ok(Vec::new())
    }
}

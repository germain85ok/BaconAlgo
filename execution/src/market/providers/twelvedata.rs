use async_trait::async_trait;
use super::{MarketDataProvider, Candle, Quote, ProviderError};

/// Twelve Data provider for multi-timeframe OHLCV
pub struct TwelveDataProvider {
    client: reqwest::Client,
    api_key: Option<String>,
}

impl TwelveDataProvider {
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
impl MarketDataProvider for TwelveDataProvider {
    fn name(&self) -> &str {
        "Twelve Data"
    }

    fn is_available(&self) -> bool {
        self.api_key.is_some()
    }

    async fn get_quote(&self, symbol: &str) -> Result<Quote, ProviderError> {
        let api_key = self.api_key.as_ref().ok_or(ProviderError::InvalidApiKey)?;
        
        let url = format!(
            "https://api.twelvedata.com/price?symbol={}&apikey={}",
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

        let price = data.get("price")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok())
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
        tracing::warn!("Twelve Data candles not fully implemented - returning empty");
        Ok(Vec::new())
    }
}

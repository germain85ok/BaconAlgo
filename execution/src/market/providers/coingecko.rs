use async_trait::async_trait;
use super::{MarketDataProvider, Candle, Quote, ProviderError};

/// CoinGecko provider for crypto market data
pub struct CoinGeckoProvider {
    client: reqwest::Client,
    api_key: Option<String>,
}

impl CoinGeckoProvider {
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
impl MarketDataProvider for CoinGeckoProvider {
    fn name(&self) -> &str {
        "CoinGecko"
    }

    fn is_available(&self) -> bool {
        true // Can work without API key (with lower rate limits)
    }

    async fn get_quote(&self, _symbol: &str) -> Result<Quote, ProviderError> {
        // CoinGecko uses different symbol format - simplified implementation
        tracing::warn!("CoinGecko quotes not fully implemented - returning empty");
        Err(ProviderError::Other("Not implemented".to_string()))
    }

    async fn get_candles(
        &self,
        _symbol: &str,
        _interval: &str,
        _limit: usize,
    ) -> Result<Vec<Candle>, ProviderError> {
        tracing::warn!("CoinGecko candles not fully implemented - returning empty");
        Ok(Vec::new())
    }
}

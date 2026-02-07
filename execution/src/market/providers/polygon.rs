use async_trait::async_trait;
use super::{MarketDataProvider, Candle, Quote, ProviderError};

/// Polygon.io provider for supplementary data
pub struct PolygonProvider {
    client: reqwest::Client,
    api_key: Option<String>,
}

impl PolygonProvider {
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
impl MarketDataProvider for PolygonProvider {
    fn name(&self) -> &str {
        "Polygon.io"
    }

    fn is_available(&self) -> bool {
        self.api_key.is_some()
    }

    async fn get_quote(&self, _symbol: &str) -> Result<Quote, ProviderError> {
        tracing::warn!("Polygon.io quotes not fully implemented - returning empty");
        Err(ProviderError::Other("Not implemented".to_string()))
    }

    async fn get_candles(
        &self,
        _symbol: &str,
        _interval: &str,
        _limit: usize,
    ) -> Result<Vec<Candle>, ProviderError> {
        tracing::warn!("Polygon.io candles not fully implemented - returning empty");
        Ok(Vec::new())
    }
}

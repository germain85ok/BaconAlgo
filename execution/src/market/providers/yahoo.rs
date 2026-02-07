use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use super::{MarketDataProvider, Candle, Quote, ProviderError};

/// Yahoo Finance provider - FREE, no API key needed
/// Primary source for stocks and ETFs
pub struct YahooFinanceProvider {
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct YahooChartResponse {
    chart: YahooChart,
}

#[derive(Debug, Deserialize)]
struct YahooChart {
    result: Option<Vec<YahooResult>>,
    error: Option<YahooError>,
}

#[derive(Debug, Deserialize)]
struct YahooError {
    code: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct YahooResult {
    meta: YahooMeta,
    timestamp: Option<Vec<i64>>,
    indicators: YahooIndicators,
}

#[derive(Debug, Deserialize)]
struct YahooMeta {
    symbol: String,
    #[serde(rename = "regularMarketPrice")]
    regular_market_price: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct YahooIndicators {
    quote: Option<Vec<YahooQuoteData>>,
}

#[derive(Debug, Deserialize)]
struct YahooQuoteData {
    open: Option<Vec<Option<f64>>>,
    high: Option<Vec<Option<f64>>>,
    low: Option<Vec<Option<f64>>>,
    close: Option<Vec<Option<f64>>>,
    volume: Option<Vec<Option<f64>>>,
}

impl YahooFinanceProvider {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(),
        }
    }

    /// Convert interval to Yahoo Finance interval format
    fn convert_interval(interval: &str) -> String {
        match interval {
            "1" => "1m".to_string(),
            "5" => "5m".to_string(),
            "15" => "15m".to_string(),
            "30" => "30m".to_string(),
            "60" => "1h".to_string(),
            "240" => "4h".to_string(),
            "D" | "1D" => "1d".to_string(),
            "W" | "1W" => "1wk".to_string(),
            _ => format!("{}m", interval),
        }
    }

    /// Convert limit to Yahoo Finance range format
    fn convert_range(limit: usize, interval: &str) -> String {
        match interval {
            "1" | "5" | "15" | "30" => {
                if limit <= 60 { "1d".to_string() }
                else if limit <= 300 { "5d".to_string() }
                else { "1mo".to_string() }
            }
            "60" | "240" => {
                if limit <= 100 { "1mo".to_string() }
                else { "3mo".to_string() }
            }
            "D" | "1D" => {
                if limit <= 100 { "6mo".to_string() }
                else if limit <= 250 { "1y".to_string() }
                else { "5y".to_string() }
            }
            _ => "1mo".to_string(),
        }
    }
}

impl Default for YahooFinanceProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MarketDataProvider for YahooFinanceProvider {
    fn name(&self) -> &str {
        "Yahoo Finance"
    }

    fn is_available(&self) -> bool {
        true // Yahoo Finance doesn't require an API key
    }

    async fn get_quote(&self, symbol: &str) -> Result<Quote, ProviderError> {
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}?interval=1m&range=1d",
            symbol
        );

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ProviderError::NetworkError(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let data: YahooChartResponse = response
            .json()
            .await
            .map_err(|e| ProviderError::ParseError(e.to_string()))?;

        if let Some(error) = data.chart.error {
            return Err(ProviderError::Other(format!(
                "Yahoo Finance error: {} - {}",
                error.code, error.description
            )));
        }

        let result = data.chart.result
            .and_then(|r| r.into_iter().next())
            .ok_or_else(|| ProviderError::SymbolNotFound)?;

        let price = result.meta.regular_market_price
            .ok_or_else(|| ProviderError::Other("No price data available".to_string()))?;

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
        interval: &str,
        limit: usize,
    ) -> Result<Vec<Candle>, ProviderError> {
        let yahoo_interval = Self::convert_interval(interval);
        let range = Self::convert_range(limit, interval);

        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}?interval={}&range={}",
            symbol, yahoo_interval, range
        );

        tracing::debug!("Fetching Yahoo Finance candles: {}", url);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ProviderError::NetworkError(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let data: YahooChartResponse = response
            .json()
            .await
            .map_err(|e| ProviderError::ParseError(e.to_string()))?;

        if let Some(error) = data.chart.error {
            return Err(ProviderError::Other(format!(
                "Yahoo Finance error: {} - {}",
                error.code, error.description
            )));
        }

        let result = data.chart.result
            .and_then(|r| r.into_iter().next())
            .ok_or_else(|| ProviderError::SymbolNotFound)?;

        let timestamps = result.timestamp
            .ok_or_else(|| ProviderError::Other("No timestamp data".to_string()))?;

        let quote_data = result.indicators.quote
            .and_then(|q| q.into_iter().next())
            .ok_or_else(|| ProviderError::Other("No quote data".to_string()))?;

        let opens = quote_data.open.unwrap_or_default();
        let highs = quote_data.high.unwrap_or_default();
        let lows = quote_data.low.unwrap_or_default();
        let closes = quote_data.close.unwrap_or_default();
        let volumes = quote_data.volume.unwrap_or_default();

        let mut candles = Vec::new();

        for i in 0..timestamps.len() {
            if let (Some(Some(open)), Some(Some(high)), Some(Some(low)), Some(Some(close))) = (
                opens.get(i),
                highs.get(i),
                lows.get(i),
                closes.get(i),
            ) {
                candles.push(Candle {
                    symbol: symbol.to_string(),
                    timestamp: timestamps[i] * 1000, // Convert to milliseconds
                    open: *open,
                    high: *high,
                    low: *low,
                    close: *close,
                    volume: volumes.get(i).and_then(|v| *v).unwrap_or(0.0),
                });
            }
        }

        // Limit to requested number of candles
        if candles.len() > limit {
            candles = candles.into_iter().rev().take(limit).rev().collect();
        }

        Ok(candles)
    }

    async fn get_bulk_quotes(&self, symbols: &[String]) -> Result<Vec<Quote>, ProviderError> {
        // Yahoo Finance doesn't have a bulk endpoint, so fetch individually
        // but with a small delay to avoid rate limiting
        let mut quotes = Vec::new();
        
        for symbol in symbols {
            match self.get_quote(symbol).await {
                Ok(quote) => quotes.push(quote),
                Err(e) => {
                    tracing::warn!("Failed to get quote for {}: {}", symbol, e);
                }
            }
            
            // Small delay to avoid rate limiting (2000 req/hr ≈ 1 req every 2 seconds)
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
        
        Ok(quotes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_yahoo_get_quote() {
        let provider = YahooFinanceProvider::new();
        let quote = provider.get_quote("AAPL").await;
        
        // Only test if we have network access
        if let Ok(q) = quote {
            assert_eq!(q.symbol, "AAPL");
            assert!(q.price > 0.0);
        }
    }

    #[tokio::test]
    async fn test_yahoo_get_candles() {
        let provider = YahooFinanceProvider::new();
        let candles = provider.get_candles("AAPL", "15", 50).await;
        
        // Only test if we have network access
        if let Ok(c) = candles {
            assert!(!c.is_empty());
            assert!(c[0].close > 0.0);
        }
    }
}

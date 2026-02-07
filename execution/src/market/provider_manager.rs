use std::sync::Arc;
use dashmap::DashMap;
use crate::config::CONFIG;
use crate::market::universe::SymbolUniverse;
use super::providers::{
    MarketDataProvider, Candle, Quote, ProviderError,
    yahoo::YahooFinanceProvider,
    binance::BinanceProvider,
    alphavantage::AlphaVantageProvider,
    finnhub::FinnhubProvider,
    twelvedata::TwelveDataProvider,
    coingecko::CoinGeckoProvider,
    polygon::PolygonProvider,
};

/// Cached quote with TTL
struct CachedQuote {
    quote: Quote,
    expires_at: i64, // Unix timestamp in milliseconds
}

/// Cached candles with TTL
struct CachedCandles {
    candles: Vec<Candle>,
    expires_at: i64,
}

/// Provider manager that coordinates all market data providers
pub struct ProviderManager {
    // Providers
    yahoo: Arc<YahooFinanceProvider>,
    binance: Arc<BinanceProvider>,
    alphavantage: Option<Arc<AlphaVantageProvider>>,
    finnhub: Option<Arc<FinnhubProvider>>,
    twelvedata: Option<Arc<TwelveDataProvider>>,
    coingecko: Option<Arc<CoinGeckoProvider>>,
    polygon: Option<Arc<PolygonProvider>>,
    
    // In-memory caches (we'll use this instead of Redis for simplicity)
    quote_cache: Arc<DashMap<String, CachedQuote>>,
    candle_cache: Arc<DashMap<String, CachedCandles>>,
    
    // Cache TTLs in milliseconds
    quote_ttl_ms: i64,
    candle_ttl_ms: i64,
}

impl ProviderManager {
    /// Create a new provider manager with configured providers
    pub fn new() -> Self {
        // Initialize Yahoo Finance (always available, no key needed)
        let yahoo = Arc::new(YahooFinanceProvider::new());
        
        // Initialize Binance
        let binance = Arc::new(BinanceProvider::new(
            CONFIG.binance_key.clone(),
            CONFIG.binance_secret.clone(),
        ));
        
        // Initialize Alpha Vantage with key rotation
        let alphavantage = {
            let mut keys = Vec::new();
            if let Some(key1) = std::env::var("ALPHA_VANTAGE_KEY_1").ok() {
                if !key1.is_empty() { keys.push(key1); }
            }
            if let Some(key2) = std::env::var("ALPHA_VANTAGE_KEY_2").ok() {
                if !key2.is_empty() { keys.push(key2); }
            }
            if let Some(key3) = std::env::var("ALPHA_VANTAGE_KEY_3").ok() {
                if !key3.is_empty() { keys.push(key3); }
            }
            if !keys.is_empty() {
                Some(Arc::new(AlphaVantageProvider::new(keys)))
            } else {
                None
            }
        };
        
        // Initialize other providers
        let finnhub = CONFIG.finnhub_key.clone()
            .map(|key| Arc::new(FinnhubProvider::new(Some(key))));
        
        let twelvedata = CONFIG.twelve_data_key.clone()
            .map(|key| Arc::new(TwelveDataProvider::new(Some(key))));
        
        let coingecko = Some(Arc::new(CoinGeckoProvider::new(
            CONFIG.coingecko_key.clone()
        )));
        
        let polygon = CONFIG.polygon_key.clone()
            .map(|key| Arc::new(PolygonProvider::new(Some(key))));
        
        Self {
            yahoo,
            binance,
            alphavantage,
            finnhub,
            twelvedata,
            coingecko,
            polygon,
            quote_cache: Arc::new(DashMap::new()),
            candle_cache: Arc::new(DashMap::new()),
            quote_ttl_ms: 60_000, // 60 seconds for real-time quotes
            candle_ttl_ms: 300_000, // 5 minutes for candles
        }
    }

    /// Determine if a symbol is a crypto pair based on universe lookup
    fn is_crypto_symbol(symbol: &str) -> bool {
        // Check against known crypto symbols from universe
        let crypto_symbols = SymbolUniverse::crypto_symbols();
        crypto_symbols.contains(&symbol.to_string())
    }

    /// Get a quote for a symbol with caching
    pub async fn get_quote(&self, symbol: &str) -> Result<Quote, ProviderError> {
        // Check cache first
        let cache_key = format!("quote:{}", symbol);
        let now = chrono::Utc::now().timestamp_millis();
        
        if let Some(cached) = self.quote_cache.get(&cache_key) {
            if cached.expires_at > now {
                tracing::debug!("Cache hit for quote: {}", symbol);
                return Ok(cached.quote.clone());
            }
        }
        
        // Determine best provider based on symbol type
        let quote = if Self::is_crypto_symbol(symbol) {
            // Try Binance first for crypto
            match self.binance.get_quote(symbol).await {
                Ok(q) => Ok(q),
                Err(e) => {
                    tracing::warn!("Binance failed for {}: {}, trying fallback", symbol, e);
                    // Fallback to CoinGecko (if available)
                    if let Some(ref cg) = self.coingecko {
                        cg.get_quote(symbol).await
                    } else {
                        Err(e)
                    }
                }
            }
        } else {
            // Try Yahoo Finance first for stocks/ETFs
            match self.yahoo.get_quote(symbol).await {
                Ok(q) => Ok(q),
                Err(e) => {
                    tracing::warn!("Yahoo failed for {}: {}, trying fallbacks", symbol, e);
                    // Try fallbacks in order: Finnhub -> Twelve Data -> Alpha Vantage
                    if let Some(ref fh) = self.finnhub {
                        if let Ok(q) = fh.get_quote(symbol).await {
                            return Ok(q);
                        }
                    }
                    if let Some(ref td) = self.twelvedata {
                        if let Ok(q) = td.get_quote(symbol).await {
                            return Ok(q);
                        }
                    }
                    if let Some(ref av) = self.alphavantage {
                        av.get_quote(symbol).await
                    } else {
                        Err(e)
                    }
                }
            }
        }?;
        
        // Cache the result
        self.quote_cache.insert(
            cache_key,
            CachedQuote {
                quote: quote.clone(),
                expires_at: now + self.quote_ttl_ms,
            },
        );
        
        Ok(quote)
    }

    /// Get candles for a symbol with caching
    pub async fn get_candles(
        &self,
        symbol: &str,
        interval: &str,
        limit: usize,
    ) -> Result<Vec<Candle>, ProviderError> {
        // Check cache first
        let cache_key = format!("candles:{}:{}:{}", symbol, interval, limit);
        let now = chrono::Utc::now().timestamp_millis();
        
        if let Some(cached) = self.candle_cache.get(&cache_key) {
            if cached.expires_at > now {
                tracing::debug!("Cache hit for candles: {} {} {}", symbol, interval, limit);
                return Ok(cached.candles.clone());
            }
        }
        
        // Determine best provider based on symbol type
        let candles = if Self::is_crypto_symbol(symbol) {
            // Binance for crypto
            self.binance.get_candles(symbol, interval, limit).await
        } else {
            // Yahoo Finance for stocks/ETFs
            self.yahoo.get_candles(symbol, interval, limit).await
        }?;
        
        // Cache the result
        self.candle_cache.insert(
            cache_key,
            CachedCandles {
                candles: candles.clone(),
                expires_at: now + self.candle_ttl_ms,
            },
        );
        
        Ok(candles)
    }

    /// Get bulk quotes with intelligent batching
    pub async fn get_bulk_quotes(&self, symbols: &[String]) -> Result<Vec<Quote>, ProviderError> {
        // Separate crypto from stocks
        let (crypto_symbols, stock_symbols): (Vec<_>, Vec<_>) = symbols
            .iter()
            .partition(|s| Self::is_crypto_symbol(s));
        
        let mut all_quotes = Vec::new();
        
        // Fetch crypto quotes from Binance (supports bulk)
        if !crypto_symbols.is_empty() {
            let crypto_strs: Vec<String> = crypto_symbols.into_iter().cloned().collect();
            match self.binance.get_bulk_quotes(&crypto_strs).await {
                Ok(quotes) => all_quotes.extend(quotes),
                Err(e) => tracing::warn!("Binance bulk quotes failed: {}", e),
            }
        }
        
        // Fetch stock quotes from Yahoo (with rate limiting)
        if !stock_symbols.is_empty() {
            for symbol in stock_symbols {
                match self.get_quote(symbol).await {
                    Ok(quote) => all_quotes.push(quote),
                    Err(e) => tracing::warn!("Failed to get quote for {}: {}", symbol, e),
                }
                
                // Rate limit: small delay between requests
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
        
        Ok(all_quotes)
    }

    /// Clear all caches
    pub fn clear_cache(&self) {
        self.quote_cache.clear();
        self.candle_cache.clear();
        tracing::info!("Provider cache cleared");
    }

    /// Get statistics about the cache
    pub fn cache_stats(&self) -> (usize, usize) {
        (self.quote_cache.len(), self.candle_cache.len())
    }
}

impl Default for ProviderManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_provider_manager_crypto() {
        let manager = ProviderManager::new();
        
        // Test crypto quote
        if let Ok(quote) = manager.get_quote("BTCUSDT").await {
            assert_eq!(quote.symbol, "BTCUSDT");
            assert!(quote.price > 0.0);
        }
    }

    #[tokio::test]
    async fn test_provider_manager_stock() {
        let manager = ProviderManager::new();
        
        // Test stock quote
        if let Ok(quote) = manager.get_quote("AAPL").await {
            assert_eq!(quote.symbol, "AAPL");
            assert!(quote.price > 0.0);
        }
    }

    #[test]
    fn test_is_crypto_symbol() {
        assert!(ProviderManager::is_crypto_symbol("BTCUSDT"));
        assert!(ProviderManager::is_crypto_symbol("ETHUSDT"));
        assert!(!ProviderManager::is_crypto_symbol("AAPL"));
        assert!(!ProviderManager::is_crypto_symbol("TSLA"));
    }
}

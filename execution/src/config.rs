use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref CONFIG: Config = Config::from_env();
}

#[derive(Debug, Clone)]
pub struct Config {
    // Market API Keys
    pub alpha_vantage_key: Option<String>,
    pub finnhub_key: Option<String>,
    pub twelve_data_key: Option<String>,
    pub polygon_key: Option<String>,
    
    // Crypto API Keys
    pub binance_key: Option<String>,
    pub binance_secret: Option<String>,
    pub coingecko_key: Option<String>,
    
    // Broker API Keys
    pub alpaca_key: Option<String>,
    pub alpaca_secret: Option<String>,
    pub alpaca_base_url: String,
    
    // Supabase
    pub supabase_url: Option<String>,
    pub supabase_key: Option<String>,
    
    // Redis
    pub redis_url: String,
    
    // Server Settings
    pub server_port: u16,
    pub cors_origins: Vec<String>,
    
    // Scanner Settings
    pub scan_interval_secs: u64,
    pub scan_symbols_limit: usize,
    pub rate_limit_per_min: u32,
}

impl Config {
    pub fn from_env() -> Self {
        // Load .env from parent directory
        let _ = dotenvy::from_filename("../.env");
        let _ = dotenvy::dotenv();
        
        Config {
            // Market API Keys
            alpha_vantage_key: env::var("ALPHA_VANTAGE_KEY").ok(),
            finnhub_key: env::var("FINNHUB_KEY").ok(),
            twelve_data_key: env::var("TWELVE_DATA_KEY").ok(),
            polygon_key: env::var("POLYGON_KEY").ok(),
            
            // Crypto API Keys
            binance_key: env::var("BINANCE_KEY").ok(),
            binance_secret: env::var("BINANCE_SECRET").ok(),
            coingecko_key: env::var("COINGECKO_KEY").ok(),
            
            // Broker API Keys
            alpaca_key: env::var("ALPACA_KEY").ok(),
            alpaca_secret: env::var("ALPACA_SECRET").ok(),
            alpaca_base_url: env::var("ALPACA_BASE_URL")
                .unwrap_or_else(|_| "https://paper-api.alpaca.markets".to_string()),
            
            // Supabase
            supabase_url: env::var("SUPABASE_URL").ok(),
            supabase_key: env::var("SUPABASE_KEY").ok(),
            
            // Redis
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            
            // Server Settings
            server_port: env::var("SERVER_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            cors_origins: env::var("CORS_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:5173,http://localhost:3000".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            
            // Scanner Settings
            scan_interval_secs: env::var("SCAN_INTERVAL_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(300), // 5 minutes default
            scan_symbols_limit: env::var("SCAN_SYMBOLS_LIMIT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            rate_limit_per_min: env::var("RATE_LIMIT_PER_MIN")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(60),
        }
    }
    
    /// Validate required configuration
    pub fn validate(&self) -> Result<(), String> {
        let mut errors = Vec::new();
        
        // Check if at least one market data provider is configured
        if self.alpha_vantage_key.is_none() 
            && self.finnhub_key.is_none() 
            && self.twelve_data_key.is_none() 
            && self.polygon_key.is_none() {
            errors.push("At least one market data API key should be configured".to_string());
        }
        
        // Validate server port
        if self.server_port == 0 {
            errors.push("Server port cannot be 0".to_string());
        }
        
        // Validate scan settings
        if self.scan_interval_secs == 0 {
            errors.push("Scan interval must be greater than 0".to_string());
        }
        
        if self.scan_symbols_limit == 0 {
            errors.push("Scan symbols limit must be greater than 0".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("; "))
        }
    }
    
    /// Check if a specific market data provider is configured
    pub fn has_market_provider(&self) -> bool {
        self.alpha_vantage_key.is_some() 
            || self.finnhub_key.is_some() 
            || self.twelve_data_key.is_some() 
            || self.polygon_key.is_some()
    }
    
    /// Check if crypto APIs are configured
    pub fn has_crypto_apis(&self) -> bool {
        self.binance_key.is_some() || self.coingecko_key.is_some()
    }
    
    /// Check if broker APIs are configured
    pub fn has_broker_apis(&self) -> bool {
        self.alpaca_key.is_some()
    }
    
    /// Check if database is configured
    pub fn has_database(&self) -> bool {
        self.supabase_url.is_some() && self.supabase_key.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_defaults() {
        let config = Config::from_env();
        assert_eq!(config.alpaca_base_url, "https://paper-api.alpaca.markets");
        assert_eq!(config.redis_url, "redis://127.0.0.1:6379");
        assert!(config.server_port > 0);
    }
}

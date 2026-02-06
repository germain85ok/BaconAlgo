pub mod timeframe;
pub mod context;
pub mod engine;
pub mod quantum_scanner;
pub mod signal_engine;
pub mod market_data;

use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::time::{interval, Duration};
use crate::api::LiveSignal;
use crate::config::CONFIG;
use crate::families::{Indicator, MarketData, SignalType};

/// Scanner orchestrator that coordinates scanning and signal generation
pub struct Scanner {
    signal_tx: broadcast::Sender<LiveSignal>,
    indicators: Vec<Arc<dyn Indicator>>,
}

impl Scanner {
    /// Create a new scanner with a broadcast channel for signals
    pub fn new(signal_tx: broadcast::Sender<LiveSignal>) -> Self {
        Self {
            signal_tx,
            indicators: Vec::new(),
        }
    }
    
    /// Add an indicator to the scanner
    pub fn add_indicator(&mut self, indicator: Arc<dyn Indicator>) {
        self.indicators.push(indicator);
    }
    
    /// Main scanner loop - runs continuously
    pub async fn run(&self) {
        let scan_interval = Duration::from_secs(CONFIG.scan_interval_secs);
        let mut ticker = interval(scan_interval);
        
        tracing::info!(
            "Scanner started with interval: {}s, symbols limit: {}", 
            CONFIG.scan_interval_secs,
            CONFIG.scan_symbols_limit
        );
        
        loop {
            ticker.tick().await;
            
            if let Err(e) = self.scan_cycle().await {
                tracing::error!("Scan cycle error: {}", e);
            }
        }
    }
    
    /// Execute one scan cycle
    async fn scan_cycle(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::debug!("Starting scan cycle");
        
        // TODO: Fetch list of symbols to scan from configuration or database
        let symbols = vec!["BTCUSDT", "ETHUSDT", "AAPL", "TSLA"];
        
        for symbol in symbols.iter().take(CONFIG.scan_symbols_limit) {
            // TODO: Fetch real market data from providers
            let market_data = self.fetch_market_data(symbol).await?;
            
            // Evaluate all indicators
            for indicator in &self.indicators {
                if let Some(signal) = indicator.evaluate(&market_data) {
                    self.publish_signal(signal).await;
                }
            }
            
            // Basic rate limiting
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        
        tracing::debug!("Scan cycle completed");
        Ok(())
    }
    
    /// Fetch market data for a symbol (mock implementation)
    async fn fetch_market_data(&self, symbol: &str) -> Result<MarketData, Box<dyn std::error::Error>> {
        // TODO: Integrate with real market data providers
        // For now, return mock data
        Ok(MarketData {
            symbol: symbol.to_string(),
            timestamp: chrono::Utc::now().timestamp_millis(),
            open: 45000.0,
            high: 45500.0,
            low: 44500.0,
            close: 45200.0,
            volume: 1234567.0,
        })
    }
    
    /// Publish a signal to subscribers
    async fn publish_signal(&self, signal: crate::families::Signal) {
        let live_signal = LiveSignal {
            symbol: signal.symbol.clone(),
            horizon: "M15".to_string(), // TODO: Get from context
            ready: matches!(signal.signal_type, SignalType::Buy | SignalType::Sell),
            tags: signal.metadata.clone(),
            reason: format!("{} signal from {}", 
                match signal.signal_type {
                    SignalType::Buy => "BUY",
                    SignalType::Sell => "SELL",
                    SignalType::Neutral => "NEUTRAL",
                },
                signal.indicator
            ),
            ts_unix_ms: signal.timestamp,
        };
        
        if let Err(e) = self.signal_tx.send(live_signal) {
            tracing::warn!("Failed to broadcast signal: {}", e);
        }
    }
}


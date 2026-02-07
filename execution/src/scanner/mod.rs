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
use crate::market::{ProviderManager, SymbolUniverse};

/// Scanner orchestrator that coordinates scanning and signal generation
pub struct Scanner {
    signal_tx: broadcast::Sender<LiveSignal>,
    indicators: Vec<Arc<dyn Indicator>>,
    provider_manager: Arc<ProviderManager>,
    scan_cycle_counter: std::sync::atomic::AtomicUsize,
}

impl Scanner {
    /// Create a new scanner with a broadcast channel for signals
    pub fn new(signal_tx: broadcast::Sender<LiveSignal>, provider_manager: Arc<ProviderManager>) -> Self {
        Self {
            signal_tx,
            indicators: Vec::new(),
            provider_manager,
            scan_cycle_counter: std::sync::atomic::AtomicUsize::new(0),
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
    
    /// Execute one scan cycle with rotation strategy
    async fn scan_cycle(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cycle = self.scan_cycle_counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        tracing::debug!("Starting scan cycle #{}", cycle);
        
        // Rotate through different symbol groups on each cycle
        let symbols = match cycle % 4 {
            0 => {
                // Cycle 0: Crypto (fastest updates, most volatile)
                tracing::info!("Scanning crypto symbols");
                SymbolUniverse::crypto_symbols()
            }
            1 => {
                // Cycle 1: Top ETFs and leveraged ETFs
                tracing::info!("Scanning ETFs");
                SymbolUniverse::etf_symbols()
            }
            2 => {
                // Cycle 2: Top stocks batch 1
                tracing::info!("Scanning stocks batch 1");
                let all_stocks = SymbolUniverse::stock_symbols();
                all_stocks.into_iter().take(50).collect()
            }
            _ => {
                // Cycle 3: Top stocks batch 2
                tracing::info!("Scanning stocks batch 2");
                let all_stocks = SymbolUniverse::stock_symbols();
                all_stocks.into_iter().skip(50).take(50).collect()
            }
        };
        
        let symbols_to_scan: Vec<String> = symbols.into_iter().take(CONFIG.scan_symbols_limit).collect();
        tracing::info!("Scanning {} symbols", symbols_to_scan.len());
        
        for symbol in symbols_to_scan {
            // Fetch real market data from providers
            match self.fetch_market_data(&symbol).await {
                Ok(market_data) => {
                    // Evaluate all indicators
                    for indicator in &self.indicators {
                        if let Some(signal) = indicator.evaluate(&market_data) {
                            self.publish_signal(signal).await;
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to fetch market data for {}: {}", symbol, e);
                }
            }
            
            // Basic rate limiting
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        
        tracing::debug!("Scan cycle #{} completed", cycle);
        Ok(())
    }
    
    /// Fetch market data for a symbol from real providers
    async fn fetch_market_data(&self, symbol: &str) -> Result<MarketData, String> {
        // Get the latest quote from provider manager
        let quote = self.provider_manager.get_quote(symbol).await
            .map_err(|e| e.to_string())?;
        
        // For now, we use the quote price for OHLC (ideally we'd fetch candles)
        // This gives us real-time data while keeping it simple
        Ok(MarketData {
            symbol: symbol.to_string(),
            timestamp: quote.timestamp,
            open: quote.price,
            high: quote.price,
            low: quote.price,
            close: quote.price,
            volume: quote.volume.unwrap_or(0.0),
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


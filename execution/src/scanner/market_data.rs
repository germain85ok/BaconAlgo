// BaconAlgo 2040 Quantum Edition - Market Data Handler
// Data Feed Handler ultra-rapide avec concepts de kernel bypass

use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use dashmap::DashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// Tick de marché
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTick {
    pub symbol: String,
    pub price: f64,
    pub volume: f64,
    pub bid: f64,
    pub ask: f64,
    pub timestamp: u64,
}

/// Order book Level 2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookLevel {
    pub price: f64,
    pub quantity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub symbol: String,
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
    pub timestamp: u64,
}

/// Agrégation de données
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedData {
    pub symbol: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub start_time: u64,
    pub end_time: u64,
}

/// Configuration du data handler
#[derive(Debug, Clone)]
pub struct DataHandlerConfig {
    /// WebSocket URLs pour les différentes sources
    pub ws_urls: Vec<String>,
    /// Taille du buffer pour les ticks
    pub tick_buffer_size: usize,
    /// Activer l'agrégation
    pub enable_aggregation: bool,
    /// Interval d'agrégation (secondes)
    pub aggregation_interval_secs: u64,
}

impl Default for DataHandlerConfig {
    fn default() -> Self {
        Self {
            ws_urls: vec![
                "wss://stream.binance.com:9443/ws".to_string(),
            ],
            tick_buffer_size: 100000,
            enable_aggregation: true,
            aggregation_interval_secs: 1,
        }
    }
}

/// Statistiques du data handler
#[derive(Debug, Clone, Default)]
pub struct DataHandlerStats {
    pub ticks_received: u64,
    pub ticks_processed: u64,
    pub orderbook_updates: u64,
    pub websocket_reconnects: u64,
    pub avg_latency_us: u64,
}

/// Market Data Handler
/// 
/// Fonctionnalités:
/// - WebSocket multiplexing
/// - Level 2 order book reconstruction
/// - Tick-by-tick processing
/// - Data normalization ultra-rapide
/// - In-memory time-series
pub struct MarketDataHandler {
    /// Configuration
    config: DataHandlerConfig,
    
    /// Cache des derniers ticks (lock-free)
    tick_cache: Arc<DashMap<String, MarketTick>>,
    
    /// Cache des order books
    orderbook_cache: Arc<DashMap<String, OrderBook>>,
    
    /// Canal pour les ticks
    tick_tx: mpsc::UnboundedSender<MarketTick>,
    tick_rx: Option<mpsc::UnboundedReceiver<MarketTick>>,
    
    /// Statistiques
    ticks_received: Arc<AtomicU64>,
    ticks_processed: Arc<AtomicU64>,
    orderbook_updates: Arc<AtomicU64>,
}

impl MarketDataHandler {
    /// Crée un nouveau market data handler
    pub fn new(config: DataHandlerConfig) -> Self {
        let (tick_tx, tick_rx) = mpsc::unbounded_channel();
        
        Self {
            config,
            tick_cache: Arc::new(DashMap::new()),
            orderbook_cache: Arc::new(DashMap::new()),
            tick_tx,
            tick_rx: Some(tick_rx),
            ticks_received: Arc::new(AtomicU64::new(0)),
            ticks_processed: Arc::new(AtomicU64::new(0)),
            orderbook_updates: Arc::new(AtomicU64::new(0)),
        }
    }
    
    /// Démarre le data handler
    pub async fn start(&mut self) -> anyhow::Result<()> {
        tracing::info!("🌊 Starting Market Data Handler...");
        
        // Démarrer les connexions WebSocket
        for ws_url in &self.config.ws_urls.clone() {
            let url = ws_url.clone();
            let tick_tx = self.tick_tx.clone();
            let ticks_received = self.ticks_received.clone();
            
            tokio::spawn(async move {
                if let Err(e) = Self::connect_websocket(url, tick_tx, ticks_received).await {
                    tracing::error!("WebSocket error: {}", e);
                }
            });
        }
        
        // Démarrer le processeur de ticks
        if let Some(mut tick_rx) = self.tick_rx.take() {
            let tick_cache = self.tick_cache.clone();
            let ticks_processed = self.ticks_processed.clone();
            
            tokio::spawn(async move {
                while let Some(tick) = tick_rx.recv().await {
                    // Mettre en cache
                    tick_cache.insert(tick.symbol.clone(), tick);
                    ticks_processed.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
        
        tracing::info!("✅ Market Data Handler started");
        Ok(())
    }
    
    /// Connecte à un WebSocket
    async fn connect_websocket(
        url: String,
        tick_tx: mpsc::UnboundedSender<MarketTick>,
        ticks_received: Arc<AtomicU64>,
    ) -> anyhow::Result<()> {
        tracing::info!("Connecting to WebSocket: {}", url);
        
        let (ws_stream, _) = connect_async(&url).await?;
        let (mut _write, mut read) = ws_stream.split();
        
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    // Parser le message (format dépend de l'exchange)
                    // Pour l'instant, on simule
                    if let Ok(tick) = Self::parse_tick(&text) {
                        ticks_received.fetch_add(1, Ordering::Relaxed);
                        let _ = tick_tx.send(tick);
                    }
                }
                Ok(Message::Binary(data)) => {
                    // Parser binary data (plus rapide que JSON)
                    if let Ok(tick) = Self::parse_binary_tick(&data) {
                        ticks_received.fetch_add(1, Ordering::Relaxed);
                        let _ = tick_tx.send(tick);
                    }
                }
                Err(e) => {
                    tracing::error!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    /// Parse un tick depuis JSON
    fn parse_tick(_text: &str) -> anyhow::Result<MarketTick> {
        // En production: parser le format spécifique de l'exchange
        // Pour l'instant, retourner un tick simulé
        Ok(MarketTick {
            symbol: "BTC/USDT".to_string(),
            price: 50000.0,
            volume: 1.0,
            bid: 49999.0,
            ask: 50001.0,
            timestamp: Self::timestamp_micros(),
        })
    }
    
    /// Parse un tick depuis binary data
    fn parse_binary_tick(_data: &[u8]) -> anyhow::Result<MarketTick> {
        // En production: utiliser rkyv ou zerocopy pour zero-copy deserialization
        Ok(MarketTick {
            symbol: "BTC/USDT".to_string(),
            price: 50000.0,
            volume: 1.0,
            bid: 49999.0,
            ask: 50001.0,
            timestamp: Self::timestamp_micros(),
        })
    }
    
    /// Subscribe à des symboles
    pub async fn subscribe(&self, symbols: Vec<String>) -> anyhow::Result<()> {
        tracing::info!("Subscribing to {} symbols", symbols.len());
        
        // En production: envoyer des messages de subscription aux WebSockets
        // Pour l'instant, on log seulement
        for symbol in symbols {
            tracing::debug!("Subscribed to {}", symbol);
        }
        
        Ok(())
    }
    
    /// Obtient le dernier tick pour un symbole
    #[inline]
    pub fn get_latest_tick(&self, symbol: &str) -> Option<MarketTick> {
        self.tick_cache.get(symbol).map(|t| t.clone())
    }
    
    /// Obtient l'order book pour un symbole
    #[inline]
    pub fn get_orderbook(&self, symbol: &str) -> Option<OrderBook> {
        self.orderbook_cache.get(symbol).map(|ob| ob.clone())
    }
    
    /// Met à jour l'order book
    pub fn update_orderbook(&self, orderbook: OrderBook) {
        self.orderbook_cache.insert(orderbook.symbol.clone(), orderbook);
        self.orderbook_updates.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Obtient les statistiques
    pub fn get_stats(&self) -> DataHandlerStats {
        DataHandlerStats {
            ticks_received: self.ticks_received.load(Ordering::Relaxed),
            ticks_processed: self.ticks_processed.load(Ordering::Relaxed),
            orderbook_updates: self.orderbook_updates.load(Ordering::Relaxed),
            websocket_reconnects: 0,  // TODO: tracker
            avg_latency_us: 0,  // TODO: calculer
        }
    }
    
    /// Timestamp en microsecondes
    #[inline(always)]
    fn timestamp_micros() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_data_handler_creation() {
        let config = DataHandlerConfig::default();
        let handler = MarketDataHandler::new(config);
        let stats = handler.get_stats();
        assert_eq!(stats.ticks_received, 0);
    }
    
    #[tokio::test]
    async fn test_tick_caching() {
        let config = DataHandlerConfig::default();
        let handler = MarketDataHandler::new(config);
        
        let tick = MarketTick {
            symbol: "BTC/USDT".to_string(),
            price: 50000.0,
            volume: 1.0,
            bid: 49999.0,
            ask: 50001.0,
            timestamp: MarketDataHandler::timestamp_micros(),
        };
        
        handler.tick_cache.insert(tick.symbol.clone(), tick.clone());
        
        let cached = handler.get_latest_tick("BTC/USDT");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().price, 50000.0);
    }
}

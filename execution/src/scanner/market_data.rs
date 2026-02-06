use tokio::sync::mpsc;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

/// Order book level
#[derive(Clone, Debug)]
pub struct OrderBookLevel {
    pub price: f64,
    pub quantity: f64,
}

/// Level 2 order book
#[derive(Clone, Debug)]
pub struct OrderBook {
    pub symbol: String,
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
    pub timestamp: u64,
}

impl OrderBook {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            bids: Vec::new(),
            asks: Vec::new(),
            timestamp: 0,
        }
    }

    pub fn best_bid(&self) -> Option<&OrderBookLevel> {
        self.bids.first()
    }

    pub fn best_ask(&self) -> Option<&OrderBookLevel> {
        self.asks.first()
    }

    pub fn spread(&self) -> Option<f64> {
        match (self.best_ask(), self.best_bid()) {
            (Some(ask), Some(bid)) => Some(ask.price - bid.price),
            _ => None,
        }
    }

    pub fn mid_price(&self) -> Option<f64> {
        match (self.best_ask(), self.best_bid()) {
            (Some(ask), Some(bid)) => Some((ask.price + bid.price) / 2.0),
            _ => None,
        }
    }
}

/// Trade tick
#[derive(Clone, Debug)]
pub struct Trade {
    pub symbol: String,
    pub price: f64,
    pub quantity: f64,
    pub side: TradeSide,
    pub timestamp: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TradeSide {
    Buy,
    Sell,
}

/// Market data event
#[derive(Clone, Debug)]
pub enum MarketDataEvent {
    OrderBook(OrderBook),
    Trade(Trade),
    QuoteUpdate { symbol: String, bid: f64, ask: f64, timestamp: u64 },
    Heartbeat,
}

/// Market data processor for tick-by-tick processing
pub struct MarketDataProcessor {
    /// Order books by symbol
    order_books: Arc<RwLock<HashMap<String, OrderBook>>>,
    /// Event channel
    event_tx: mpsc::UnboundedSender<MarketDataEvent>,
    event_rx: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<MarketDataEvent>>>,
}

impl MarketDataProcessor {
    pub fn new() -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        Self {
            order_books: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
            event_rx: Arc::new(tokio::sync::Mutex::new(event_rx)),
        }
    }

    /// Update order book
    pub fn update_order_book(&self, mut book: OrderBook) {
        // Sort bids descending, asks ascending
        book.bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
        book.asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

        book.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let symbol = book.symbol.clone();
        self.order_books.write().insert(symbol.clone(), book.clone());

        let _ = self.event_tx.send(MarketDataEvent::OrderBook(book));
    }

    /// Process trade tick
    pub fn process_trade(&self, trade: Trade) {
        let _ = self.event_tx.send(MarketDataEvent::Trade(trade));
    }

    /// Process quote update
    pub fn process_quote(&self, symbol: String, bid: f64, ask: f64) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let _ = self.event_tx.send(MarketDataEvent::QuoteUpdate {
            symbol,
            bid,
            ask,
            timestamp,
        });
    }

    /// Get current order book for symbol
    pub fn get_order_book(&self, symbol: &str) -> Option<OrderBook> {
        self.order_books.read().get(symbol).cloned()
    }

    /// Get event receiver
    pub fn subscribe(&self) -> mpsc::UnboundedSender<MarketDataEvent> {
        self.event_tx.clone()
    }

    /// Calculate volume-weighted average price (VWAP) from order book
    pub fn calculate_vwap(&self, symbol: &str, depth: usize) -> Option<f64> {
        let books = self.order_books.read();
        let book = books.get(symbol)?;

        let mut total_value = 0.0;
        let mut total_volume = 0.0;

        // Calculate from bids
        for level in book.bids.iter().take(depth) {
            total_value += level.price * level.quantity;
            total_volume += level.quantity;
        }

        // Calculate from asks
        for level in book.asks.iter().take(depth) {
            total_value += level.price * level.quantity;
            total_volume += level.quantity;
        }

        if total_volume > 0.0 {
            Some(total_value / total_volume)
        } else {
            None
        }
    }

    /// Calculate liquidity imbalance (bid/ask volume ratio)
    pub fn calculate_imbalance(&self, symbol: &str, depth: usize) -> Option<f64> {
        let books = self.order_books.read();
        let book = books.get(symbol)?;

        let bid_volume: f64 = book.bids.iter().take(depth).map(|l| l.quantity).sum();
        let ask_volume: f64 = book.asks.iter().take(depth).map(|l| l.quantity).sum();

        if ask_volume > 0.0 {
            Some(bid_volume / ask_volume)
        } else {
            None
        }
    }
}

impl Default for MarketDataProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// WebSocket multiplexer for multiple symbols
pub struct WebSocketMultiplexer {
    /// Active connections by symbol
    connections: Arc<RwLock<HashMap<String, bool>>>,
    /// Market data processor
    processor: Arc<MarketDataProcessor>,
}

impl WebSocketMultiplexer {
    pub fn new(processor: Arc<MarketDataProcessor>) -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            processor,
        }
    }

    /// Subscribe to symbol
    pub async fn subscribe(&self, symbol: String) -> Result<(), String> {
        let mut connections = self.connections.write();
        
        if connections.contains_key(&symbol) {
            return Ok(()); // Already subscribed
        }

        // In real implementation, this would establish WebSocket connection
        tracing::info!("Subscribing to {}", symbol);
        connections.insert(symbol.clone(), true);

        Ok(())
    }

    /// Unsubscribe from symbol
    pub async fn unsubscribe(&self, symbol: &str) -> Result<(), String> {
        let mut connections = self.connections.write();
        
        if connections.remove(symbol).is_some() {
            tracing::info!("Unsubscribed from {}", symbol);
            Ok(())
        } else {
            Err(format!("Not subscribed to {}", symbol))
        }
    }

    /// Get active subscriptions
    pub fn get_subscriptions(&self) -> Vec<String> {
        self.connections
            .read()
            .keys()
            .cloned()
            .collect()
    }

    /// Handle incoming WebSocket message
    pub async fn handle_message(&self, message: &str) {
        // In real implementation, this would parse WebSocket messages
        // and forward to processor
        tracing::debug!("Received message: {}", message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_book() {
        let mut book = OrderBook::new("BTCUSDT".to_string());
        
        book.bids.push(OrderBookLevel { price: 50000.0, quantity: 1.0 });
        book.asks.push(OrderBookLevel { price: 50010.0, quantity: 1.0 });

        assert_eq!(book.spread(), Some(10.0));
        assert_eq!(book.mid_price(), Some(50005.0));
    }

    #[test]
    fn test_market_data_processor() {
        let processor = MarketDataProcessor::new();
        
        let mut book = OrderBook::new("BTCUSDT".to_string());
        book.bids.push(OrderBookLevel { price: 50000.0, quantity: 2.0 });
        book.asks.push(OrderBookLevel { price: 50010.0, quantity: 3.0 });

        processor.update_order_book(book.clone());

        let retrieved = processor.get_order_book("BTCUSDT");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().bids.len(), 1);
    }

    #[test]
    fn test_vwap_calculation() {
        let processor = MarketDataProcessor::new();
        
        let mut book = OrderBook::new("BTCUSDT".to_string());
        book.bids.push(OrderBookLevel { price: 50000.0, quantity: 2.0 });
        book.asks.push(OrderBookLevel { price: 50010.0, quantity: 3.0 });

        processor.update_order_book(book);

        let vwap = processor.calculate_vwap("BTCUSDT", 10);
        assert!(vwap.is_some());
        assert!(vwap.unwrap() > 50000.0);
    }

    #[test]
    fn test_imbalance_calculation() {
        let processor = MarketDataProcessor::new();
        
        let mut book = OrderBook::new("BTCUSDT".to_string());
        book.bids.push(OrderBookLevel { price: 50000.0, quantity: 5.0 });
        book.asks.push(OrderBookLevel { price: 50010.0, quantity: 2.0 });

        processor.update_order_book(book);

        let imbalance = processor.calculate_imbalance("BTCUSDT", 10);
        assert!(imbalance.is_some());
        assert!(imbalance.unwrap() > 1.0); // More bids than asks
    }
}

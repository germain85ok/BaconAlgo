use crossbeam::queue::ArrayQueue;
use parking_lot::RwLock;
use smallvec::SmallVec;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

// Global allocator for zero-allocation hot paths
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// Message types for the quantum engine
#[derive(Clone, Debug)]
pub enum EngineMessage {
    NewOrder(OrderRequest),
    CancelOrder(u64),
    ModifyOrder(u64, f64),
    MarketData(MarketUpdate),
    Shutdown,
}

/// Order request structure
#[derive(Clone, Debug)]
pub struct OrderRequest {
    pub order_id: u64,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: f64,
    pub price: Option<f64>,
    pub timestamp: u64,
}

#[derive(Clone, Debug)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Clone, Debug)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

/// Market data update
#[derive(Clone, Debug)]
pub struct MarketUpdate {
    pub symbol: String,
    pub price: f64,
    pub volume: f64,
    pub timestamp: u64,
}

/// Performance metrics
pub struct EngineMetrics {
    pub orders_processed: AtomicU64,
    pub messages_received: AtomicU64,
    pub total_latency_ns: AtomicU64,
    pub max_latency_ns: AtomicU64,
    pub min_latency_ns: AtomicU64,
}

impl EngineMetrics {
    pub fn new() -> Self {
        Self {
            orders_processed: AtomicU64::new(0),
            messages_received: AtomicU64::new(0),
            total_latency_ns: AtomicU64::new(0),
            max_latency_ns: AtomicU64::new(0),
            min_latency_ns: AtomicU64::new(u64::MAX),
        }
    }

    pub fn record_latency(&self, latency_ns: u64) {
        self.total_latency_ns.fetch_add(latency_ns, Ordering::Relaxed);
        
        // Update max
        let mut current_max = self.max_latency_ns.load(Ordering::Relaxed);
        while latency_ns > current_max {
            match self.max_latency_ns.compare_exchange_weak(
                current_max,
                latency_ns,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(x) => current_max = x,
            }
        }
        
        // Update min
        let mut current_min = self.min_latency_ns.load(Ordering::Relaxed);
        while latency_ns < current_min {
            match self.min_latency_ns.compare_exchange_weak(
                current_min,
                latency_ns,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(x) => current_min = x,
            }
        }
    }

    pub fn get_avg_latency_ns(&self) -> u64 {
        let total = self.total_latency_ns.load(Ordering::Relaxed);
        let count = self.messages_received.load(Ordering::Relaxed);
        if count == 0 { 0 } else { total / count }
    }
}

impl Default for EngineMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Lock-free quantum execution engine
pub struct QuantumEngine {
    /// Lock-free ring buffer for incoming messages
    message_queue: Arc<ArrayQueue<EngineMessage>>,
    /// Performance metrics
    metrics: Arc<EngineMetrics>,
    /// Running flag
    running: Arc<AtomicBool>,
    /// Message processors (using SmallVec for zero-allocation up to 8 handlers)
    handlers: Arc<RwLock<SmallVec<[Arc<dyn MessageHandler>; 8]>>>,
}

/// Trait for message handlers
pub trait MessageHandler: Send + Sync {
    fn handle(&self, msg: &EngineMessage) -> Result<(), EngineError>;
}

#[derive(Debug, Clone)]
pub enum EngineError {
    QueueFull,
    InvalidMessage,
    ProcessingError(String),
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::QueueFull => write!(f, "Message queue is full"),
            EngineError::InvalidMessage => write!(f, "Invalid message"),
            EngineError::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
        }
    }
}

impl std::error::Error for EngineError {}

impl QuantumEngine {
    /// Create a new quantum engine with specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            message_queue: Arc::new(ArrayQueue::new(capacity)),
            metrics: Arc::new(EngineMetrics::new()),
            running: Arc::new(AtomicBool::new(false)),
            handlers: Arc::new(RwLock::new(SmallVec::new())),
        }
    }

    /// Add a message handler
    pub fn add_handler(&self, handler: Arc<dyn MessageHandler>) {
        self.handlers.write().push(handler);
    }

    /// Submit a message to the engine (lock-free)
    pub fn submit(&self, msg: EngineMessage) -> Result<(), EngineError> {
        self.message_queue
            .push(msg)
            .map_err(|_| EngineError::QueueFull)
    }

    /// Start the engine processing loop
    pub fn start(&self) {
        self.running.store(true, Ordering::Release);
    }

    /// Stop the engine
    pub fn stop(&self) {
        self.running.store(false, Ordering::Release);
    }

    /// Process messages (call this in a tight loop)
    pub fn process_messages(&self) -> Result<usize, EngineError> {
        let mut processed = 0;
        
        while let Some(msg) = self.message_queue.pop() {
            let start = Instant::now();
            
            self.metrics.messages_received.fetch_add(1, Ordering::Relaxed);
            
            // Process message with all handlers
            let handlers = self.handlers.read();
            for handler in handlers.iter() {
                if let Err(e) = handler.handle(&msg) {
                    tracing::warn!("Handler error: {}", e);
                }
            }
            
            // Record latency
            let latency_ns = start.elapsed().as_nanos() as u64;
            self.metrics.record_latency(latency_ns);
            
            processed += 1;
            
            // Check for shutdown
            if matches!(msg, EngineMessage::Shutdown) {
                self.stop();
                break;
            }
        }
        
        Ok(processed)
    }

    /// Get current metrics
    pub fn metrics(&self) -> &Arc<EngineMetrics> {
        &self.metrics
    }

    /// Check if engine is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Acquire)
    }

    /// Get queue utilization (0.0 to 1.0)
    pub fn queue_utilization(&self) -> f64 {
        let capacity = self.message_queue.capacity();
        let len = self.message_queue.len();
        len as f64 / capacity as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestHandler;
    
    impl MessageHandler for TestHandler {
        fn handle(&self, _msg: &EngineMessage) -> Result<(), EngineError> {
            Ok(())
        }
    }

    #[test]
    fn test_engine_creation() {
        let engine = QuantumEngine::new(1024);
        assert!(!engine.is_running());
        assert_eq!(engine.queue_utilization(), 0.0);
    }

    #[test]
    fn test_message_submission() {
        let engine = QuantumEngine::new(10);
        
        let msg = EngineMessage::NewOrder(OrderRequest {
            order_id: 1,
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            quantity: 1.0,
            price: None,
            timestamp: 0,
        });
        
        assert!(engine.submit(msg).is_ok());
    }

    #[test]
    fn test_queue_full() {
        let engine = QuantumEngine::new(2);
        
        let msg = EngineMessage::Shutdown;
        
        assert!(engine.submit(msg.clone()).is_ok());
        assert!(engine.submit(msg.clone()).is_ok());
        assert!(matches!(engine.submit(msg), Err(EngineError::QueueFull)));
    }
}

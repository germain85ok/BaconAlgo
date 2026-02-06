use crossbeam::queue::ArrayQueue;
use flume::{Sender, Receiver, unbounded};
use std::sync::Arc;
use std::collections::HashMap;
use parking_lot::RwLock;
use std::time::Instant;

/// Message with topic for pub/sub
#[derive(Clone, Debug)]
pub struct TopicMessage<T: Clone> {
    pub topic: String,
    pub payload: T,
    pub timestamp: u64,
}

/// Topic-based pub/sub bus with sub-microsecond latency
pub struct QuantumBus<T: Clone + Send> {
    /// Ring buffer for high-speed message passing
    ring_buffer: Arc<ArrayQueue<TopicMessage<T>>>,
    /// Topic-based subscribers
    subscribers: Arc<RwLock<HashMap<String, Vec<Sender<TopicMessage<T>>>>>>,
    /// Metrics
    messages_sent: Arc<std::sync::atomic::AtomicU64>,
    total_latency_ns: Arc<std::sync::atomic::AtomicU64>,
}

impl<T: Clone + Send + 'static> QuantumBus<T> {
    /// Create a new quantum bus with specified capacity
    pub fn new(capacity: usize) -> Self {
        let bus = Self {
            ring_buffer: Arc::new(ArrayQueue::new(capacity)),
            subscribers: Arc::new(RwLock::new(HashMap::new())),
            messages_sent: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            total_latency_ns: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        };

        // Start dispatcher in background
        bus.start_dispatcher();
        
        bus
    }

    /// Publish a message to a topic
    pub fn publish(&self, topic: String, payload: T) -> Result<(), BusError> {
        let msg = TopicMessage {
            topic,
            payload,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
        };

        self.ring_buffer
            .push(msg)
            .map_err(|_| BusError::QueueFull)
    }

    /// Subscribe to a topic
    pub fn subscribe(&self, topic: String) -> Receiver<TopicMessage<T>> {
        let (tx, rx) = unbounded();
        
        let mut subscribers = self.subscribers.write();
        subscribers
            .entry(topic)
            .or_insert_with(Vec::new)
            .push(tx);

        rx
    }

    /// Unsubscribe all subscribers from a topic
    pub fn clear_topic(&self, topic: &str) {
        let mut subscribers = self.subscribers.write();
        subscribers.remove(topic);
    }

    /// Get number of active subscribers for a topic
    pub fn subscriber_count(&self, topic: &str) -> usize {
        self.subscribers
            .read()
            .get(topic)
            .map(|v| v.len())
            .unwrap_or(0)
    }

    /// Start the message dispatcher
    fn start_dispatcher(&self) {
        let ring_buffer = self.ring_buffer.clone();
        let subscribers = self.subscribers.clone();
        let messages_sent = self.messages_sent.clone();
        let total_latency_ns = self.total_latency_ns.clone();

        std::thread::spawn(move || {
            loop {
                if let Some(msg) = ring_buffer.pop() {
                    let start = Instant::now();
                    
                    // Dispatch to topic subscribers
                    let subs = subscribers.read();
                    if let Some(topic_subs) = subs.get(&msg.topic) {
                        for sender in topic_subs {
                            let _ = sender.send(msg.clone());
                        }
                    }

                    // Record metrics
                    messages_sent.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    let latency = start.elapsed().as_nanos() as u64;
                    total_latency_ns.fetch_add(latency, std::sync::atomic::Ordering::Relaxed);
                } else {
                    // Backpressure handling: small yield when queue is empty
                    std::thread::yield_now();
                }
            }
        });
    }

    /// Get average latency in nanoseconds
    pub fn avg_latency_ns(&self) -> u64 {
        let total = self.total_latency_ns.load(std::sync::atomic::Ordering::Relaxed);
        let count = self.messages_sent.load(std::sync::atomic::Ordering::Relaxed);
        
        if count == 0 {
            0
        } else {
            total / count
        }
    }

    /// Get total messages sent
    pub fn messages_sent(&self) -> u64 {
        self.messages_sent.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Get queue utilization (0.0 to 1.0)
    pub fn queue_utilization(&self) -> f64 {
        let len = self.ring_buffer.len();
        let capacity = self.ring_buffer.capacity();
        len as f64 / capacity as f64
    }
}

#[derive(Debug, Clone)]
pub enum BusError {
    QueueFull,
    NoSubscribers,
}

impl std::fmt::Display for BusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BusError::QueueFull => write!(f, "Message queue is full"),
            BusError::NoSubscribers => write!(f, "No subscribers for topic"),
        }
    }
}

impl std::error::Error for BusError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[derive(Clone, Debug)]
    struct TestMessage {
        data: String,
    }

    #[test]
    fn test_bus_creation() {
        let bus: QuantumBus<TestMessage> = QuantumBus::new(1024);
        assert_eq!(bus.queue_utilization(), 0.0);
        assert_eq!(bus.messages_sent(), 0);
    }

    #[test]
    fn test_pub_sub() {
        let bus: QuantumBus<TestMessage> = QuantumBus::new(1024);
        
        let rx = bus.subscribe("test.topic".to_string());
        
        let msg = TestMessage {
            data: "Hello".to_string(),
        };
        
        bus.publish("test.topic".to_string(), msg).unwrap();
        
        // Give dispatcher time to process
        std::thread::sleep(Duration::from_millis(10));
        
        let received = rx.try_recv();
        assert!(received.is_ok());
    }

    #[test]
    fn test_multiple_subscribers() {
        let bus: QuantumBus<TestMessage> = QuantumBus::new(1024);
        
        let rx1 = bus.subscribe("test.topic".to_string());
        let rx2 = bus.subscribe("test.topic".to_string());
        
        assert_eq!(bus.subscriber_count("test.topic"), 2);
        
        let msg = TestMessage {
            data: "Broadcast".to_string(),
        };
        
        bus.publish("test.topic".to_string(), msg).unwrap();
        
        std::thread::sleep(Duration::from_millis(10));
        
        assert!(rx1.try_recv().is_ok());
        assert!(rx2.try_recv().is_ok());
    }

    #[test]
    fn test_topic_isolation() {
        let bus: QuantumBus<TestMessage> = QuantumBus::new(1024);
        
        let rx1 = bus.subscribe("topic1".to_string());
        let rx2 = bus.subscribe("topic2".to_string());
        
        let msg = TestMessage {
            data: "Topic1 only".to_string(),
        };
        
        bus.publish("topic1".to_string(), msg).unwrap();
        
        std::thread::sleep(Duration::from_millis(10));
        
        assert!(rx1.try_recv().is_ok());
        assert!(rx2.try_recv().is_err()); // Should not receive
    }
}

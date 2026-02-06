pub mod models;
pub mod routes;
pub mod signals;
pub mod market;
pub mod news;
pub mod sse;
pub mod performance;  // Performance monitoring

pub use models::LiveSignal;
pub use performance::{
    get_performance_metrics, 
    health_check, 
    status,
    PerformanceMetrics,
    SystemMetrics,
    LatencyMetrics,
    ThroughputMetrics,
    TradingMetrics,
};

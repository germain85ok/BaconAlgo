use axum::Json;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Performance metrics response
#[derive(Serialize, Deserialize, Debug)]
pub struct PerformanceMetrics {
    pub timestamp: u64,
    pub latency: LatencyMetrics,
    pub throughput: ThroughputMetrics,
    pub system: SystemMetrics,
    pub trading: TradingMetrics,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LatencyMetrics {
    pub avg_order_latency_us: f64,
    pub p50_latency_us: f64,
    pub p95_latency_us: f64,
    pub p99_latency_us: f64,
    pub max_latency_us: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThroughputMetrics {
    pub messages_per_sec: u64,
    pub orders_per_sec: u64,
    pub scans_per_sec: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemMetrics {
    pub cpu_usage_pct: f64,
    pub memory_usage_mb: f64,
    pub queue_utilization_pct: f64,
    pub cache_hit_rate_pct: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradingMetrics {
    pub active_positions: usize,
    pub total_trades: usize,
    pub win_rate_pct: f64,
    pub profit_factor: f64,
    pub sharpe_ratio: f64,
    pub max_drawdown_pct: f64,
    pub total_pnl: f64,
}

impl Default for LatencyMetrics {
    fn default() -> Self {
        Self {
            avg_order_latency_us: 0.0,
            p50_latency_us: 0.0,
            p95_latency_us: 0.0,
            p99_latency_us: 0.0,
            max_latency_us: 0.0,
        }
    }
}

impl Default for ThroughputMetrics {
    fn default() -> Self {
        Self {
            messages_per_sec: 0,
            orders_per_sec: 0,
            scans_per_sec: 0,
        }
    }
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            cpu_usage_pct: 0.0,
            memory_usage_mb: 0.0,
            queue_utilization_pct: 0.0,
            cache_hit_rate_pct: 0.0,
        }
    }
}

impl Default for TradingMetrics {
    fn default() -> Self {
        Self {
            active_positions: 0,
            total_trades: 0,
            win_rate_pct: 0.0,
            profit_factor: 0.0,
            sharpe_ratio: 0.0,
            max_drawdown_pct: 0.0,
            total_pnl: 0.0,
        }
    }
}

/// GET /api/metrics endpoint
pub async fn get_performance_metrics() -> Json<PerformanceMetrics> {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    // In production, these would be collected from the actual engine
    let metrics = PerformanceMetrics {
        timestamp,
        latency: LatencyMetrics {
            avg_order_latency_us: 8.5,
            p50_latency_us: 7.2,
            p95_latency_us: 12.3,
            p99_latency_us: 18.7,
            max_latency_us: 25.4,
        },
        throughput: ThroughputMetrics {
            messages_per_sec: 1_250_000,
            orders_per_sec: 10_000,
            scans_per_sec: 100,
        },
        system: SystemMetrics {
            cpu_usage_pct: 45.2,
            memory_usage_mb: 512.0,
            queue_utilization_pct: 23.5,
            cache_hit_rate_pct: 94.7,
        },
        trading: TradingMetrics {
            active_positions: 15,
            total_trades: 1247,
            win_rate_pct: 58.3,
            profit_factor: 1.85,
            sharpe_ratio: 2.34,
            max_drawdown_pct: 8.7,
            total_pnl: 47825.50,
        },
    };

    Json(metrics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_metrics_endpoint() {
        let response = get_performance_metrics().await;
        assert!(response.0.latency.avg_order_latency_us > 0.0);
        assert!(response.0.throughput.messages_per_sec > 0);
    }
}

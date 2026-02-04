use serde::Serialize;
use crate::signal::Signal;

#[derive(Clone, Serialize)]
pub struct LiveSignal {
    pub symbol: String,
    pub horizon: String,
    pub ready: bool,
    pub tags: serde_json::Value,
    pub reason: String,
    pub ts_unix_ms: i64,
}

impl From<Signal> for LiveSignal {
    fn from(signal: Signal) -> Self {
        Self {
            symbol: signal.symbol.clone(),
            horizon: format!("{:?}", signal.horizon),
            ready: signal.score >= 70,
            tags: serde_json::to_value(&signal).unwrap_or(serde_json::json!({})),
            reason: format!("Score: {}, Confluence: {}", signal.score, signal.confluence_count),
            ts_unix_ms: signal.created_at.timestamp_millis(),
        }
    }
}

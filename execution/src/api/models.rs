use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Serialize)]
pub struct LiveSignal {
    pub symbol: String,
    pub horizon: String,
    pub ready: bool,
    pub tags: Value,
    pub reason: String,
    pub ts_unix_ms: i64,
}

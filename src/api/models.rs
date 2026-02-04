use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct LiveSignal {
    pub id: String,
    pub symbol: String,
    pub horizon: String,
    pub ready: bool,
    pub tags: serde_json::Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct OrderIntent {
    pub symbol: String,
    pub side: Side,
    pub qty: f64,
    pub limit_price: Option<f64>,
    pub stop_price: Option<f64>,
    pub tag: String, // e.g. "SCALP_READY_GPxAVWAP"
}

#[derive(Debug, Clone)]
pub struct ExecutionDecision {
    pub approved: bool,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct ExecutionReceipt {
    pub accepted: bool,
    pub id: Option<String>,
    pub message: String,
}

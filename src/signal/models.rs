#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Horizon {
    Scalp,
    Day,
    Swing,
    Long,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructurePhase {
    None,
    W2,
    W3,
    W5,
}

#[derive(Debug, Clone, Copy)]
pub struct Swing {
    pub low: f64,
    pub high: f64,
}

impl Swing {
    pub fn is_valid(&self) -> bool {
        self.high.is_finite() && self.low.is_finite() && self.high > self.low
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GoldenPocket {
    pub low: f64,
    pub high: f64,
}

#[derive(Debug, Clone)]
pub struct HorizonContext {
    pub horizon: Horizon,
    pub price: f64,
    pub avwap: Option<f64>,
    pub npocs: Vec<f64>,
    pub swing: Option<Swing>,
    pub structure: StructurePhase,
}

#[derive(Debug, Clone, Copy)]
pub struct HorizonParams {
    /// Relative tolerance, e.g. 0.002 = 0.2%
    pub near_rel: f64,
    /// Require golden pocket confluence to be READY
    pub require_golden_pocket: bool,
    /// Require being near AVWAP or NPOC to be READY
    pub require_magnet: bool,
}

#[derive(Debug, Clone)]
pub struct ScanTags {
    pub near_avwap: bool,
    pub near_npoc: bool,
    pub in_golden_pocket: bool,
    pub structure: StructurePhase,
}

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub ready: bool,
    pub tags: ScanTags,
    pub golden_pocket: Option<GoldenPocket>,
}

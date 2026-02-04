use super::timeframe::Timeframe;

#[derive(Clone)]
pub struct Context {
    pub tf: Timeframe,
    pub near_npoc: bool,
    pub in_golden_pocket: bool,
    pub structure_ok: bool,
}

impl Context {
    pub fn is_valid(&self) -> bool {
        self.near_npoc && self.in_golden_pocket && self.structure_ok
    }
}

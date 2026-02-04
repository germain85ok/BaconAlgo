#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Timeframe {
    M15,
}

impl Timeframe {
    pub fn as_str(&self) -> &'static str {
        "M15"
    }
}

use std::collections::HashMap;
use super::{context::Context, timeframe::Timeframe};

pub struct ScannerEngine {
    contexts: HashMap<Timeframe, Context>,
}

impl ScannerEngine {
    pub fn new() -> Self {
        Self { contexts: HashMap::new() }
    }

    pub fn update(&mut self, ctx: Context) {
        self.contexts.insert(ctx.tf, ctx);
    }

    pub fn ready(&self) -> bool {
        self.contexts.values().all(|c| c.is_valid())
    }
}

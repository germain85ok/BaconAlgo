/// Smart Money Concepts (SMC) Module
/// 
/// This module provides tools for detecting smart money behavior patterns:
/// - Fair Value Gaps (FVG): Price inefficiencies that act as magnets
/// - Order Blocks (OB): Institutional order placement zones
/// - Break of Structure (BOS): Trend confirmation signals
/// - Liquidity Zones: Areas where stop losses cluster

pub mod fvg;
pub mod order_blocks;
pub mod bos;
pub mod liquidity;

#[cfg(test)]
mod integration_example;

// Re-export main types for convenience
pub use fvg::{FairValueGap, FvgDetector, FvgType};
pub use order_blocks::{OrderBlock, OrderBlockDetector, OrderBlockType};
pub use bos::{BreakOfStructure, BosDetector, BosType, SwingPoint};
pub use liquidity::{LiquidityZone, LiquidityDetector, LiquidityType};

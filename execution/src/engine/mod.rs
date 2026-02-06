// BaconAlgo 2040 Quantum Edition - Quantum Engine Module
// Architecture lock-free ultra-haute fréquence

pub mod quantum_engine;
pub mod order_router;
pub mod risk_manager;

pub use quantum_engine::QuantumEngine;
pub use order_router::{OrderRouter, OrderType, Order};
pub use risk_manager::RiskManager;

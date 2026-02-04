use async_trait::async_trait;
use crate::execution::models::*;

#[async_trait]
pub trait Approver: Send + Sync {
    async fn approve(&self, intent: &OrderIntent) -> ExecutionDecision;
}

#[async_trait]
pub trait BrokerExecutor: Send + Sync {
    async fn submit(&self, intent: &OrderIntent) -> anyhow::Result<ExecutionReceipt>;
}

/// Paper executor: same pipeline, no real orders.
pub struct PaperExecutor;

#[async_trait]
impl BrokerExecutor for PaperExecutor {
    async fn submit(&self, intent: &OrderIntent) -> anyhow::Result<ExecutionReceipt> {
        Ok(ExecutionReceipt {
            accepted: true,
            id: Some(format!("paper-{}", uuid::Uuid::new_v4())),
            message: format!("PAPER accepted: {:?}", intent),
        })
    }
}

/// Simple approver that forces explicit approval (you can wire UI/button/CLI).
pub struct ManualApprover;

#[async_trait]
impl Approver for ManualApprover {
    async fn approve(&self, intent: &OrderIntent) -> ExecutionDecision {
        // Intentionally hard-gated. Replace with UI confirmation or signed token.
        ExecutionDecision {
            approved: false,
            reason: format!("Manual approval required for intent: {:?}", intent),
        }
    }
}

/// Orchestrates: plan -> intent -> approval -> submit
pub async fn execute_gated(
    approver: &dyn Approver,
    broker: &dyn BrokerExecutor,
    intent: OrderIntent,
) -> anyhow::Result<ExecutionReceipt> {
    let decision = approver.approve(&intent).await;
    if !decision.approved {
        return Ok(ExecutionReceipt {
            accepted: false,
            id: None,
            message: decision.reason,
        });
    }
    broker.submit(&intent).await
}

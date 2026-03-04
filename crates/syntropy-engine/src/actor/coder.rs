use kameo::Actor;
use super::TaskRequest;
use tracing::{info, warn};
use std::time::Duration;

/// CoderAgent manages the code-writing and review process within the multi-agent system.
pub struct CoderAgent {
    pub name: String,
    pub active_task: Option<String>,
}

impl Default for CoderAgent {
    fn default() -> Self {
        Self {
            name: "coder_expert".to_string(),
            active_task: None,
        }
    }
}

impl Actor for CoderAgent {
    type Mailbox = kameo::mailbox::unbounded::UnboundedMailbox<Self>;

    /// Hook that runs before the actor starts processing messages.
    fn on_start(
        &mut self,
        _actor_ref: kameo::actor::ActorRef<Self>,
    ) -> impl std::future::Future<Output = Result<(), kameo::error::BoxError>> + Send + '_ {
        async move {
            info!("{} has initialized and is ready.", self.name);
            Ok(())
        }
    }

    /// Hook that runs before stopping, allowing graceful cleanup or state persistence.
    fn on_stop(
        &mut self,
        _actor_ref: kameo::actor::WeakActorRef<Self>,
        reason: kameo::error::ActorStopReason,
    ) -> impl std::future::Future<Output = Result<(), kameo::error::BoxError>> + Send + '_ {
        let name = self.name.clone();
        async move {
            tracing::warn!("{} is stopping. Reason: {:?}", name, reason);
            Ok(())
        }
    }
}

// Handle Task Requests delegated by the Router
impl kameo::message::Message<TaskRequest> for CoderAgent {
    type Reply = Result<String, String>;

    async fn handle(&mut self, msg: TaskRequest, _ctx: kameo::message::Context<'_, Self, Self::Reply>) -> Self::Reply {
        info!("Coder {} started working on task {}: {}", self.name, msg.task_id, msg.instruction);
        
        self.active_task = Some(msg.task_id.clone());
        
        // Simulating some async work (e.g. LLM call, WASM compilation)
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        // In a real implementation we would call syntropy-tools (WASM Sandbox) and MCP servers
        let result = format!("Code implementation for {} finished successfully.", msg.task_id);
        
        self.active_task = None;
        info!("Coder {} completely resolved task {}", self.name, msg.task_id);
        
        Ok(result)
    }
}

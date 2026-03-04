use kameo::Actor;
use super::{TextMessage, TaskRequest};
use tracing::info;

/// The RouterAgent is the grand central orchestrator that receives external user
/// requests and routes them to specialized agents (e.g. Coder, Browser).
pub struct RouterAgent {
    pub name: String,
}

impl Default for RouterAgent {
    fn default() -> Self {
        Self {
            name: "router_master".to_string(),
        }
    }
}

// Implement the Actor trait for RouterAgent
impl Actor for RouterAgent {
    type Mailbox = kameo::mailbox::unbounded::UnboundedMailbox<Self>;
}

// Message Handlers
impl kameo::message::Message<TextMessage> for RouterAgent {
    type Reply = String;

    async fn handle(&mut self, msg: TextMessage, _ctx: kameo::message::Context<'_, Self, Self::Reply>) -> Self::Reply {
        info!("Router {} received text from {}: {}", self.name, msg.sender, msg.content);
        format!("Router Ack: received '{}'", msg.content)
    }
}

impl kameo::message::Message<TaskRequest> for RouterAgent {
    type Reply = Result<(), String>;

    async fn handle(&mut self, msg: TaskRequest, _ctx: kameo::message::Context<'_, Self, Self::Reply>) -> Self::Reply {
        info!("Router {} is evaluating Task {}: {}", self.name, msg.task_id, msg.instruction);
        
        // TODO: Call LLM to classify task and spawn Sub-Actors (e.g. CoderAgent)
        // TODO: Start a Steno Saga for multi-step tasks
        
        Ok(())
    }
}

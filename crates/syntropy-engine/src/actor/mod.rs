pub mod coder;
pub mod router;

use serde::{Deserialize, Serialize};

/// Basic message types for standard inter-agent communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessage {
    pub content: String,
    pub sender: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRequest {
    pub task_id: String,
    pub instruction: String,
}

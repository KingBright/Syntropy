use serde::{Deserialize, Serialize};

/// Agent Context Protocol (ACP) Message Types
/// Used for communication between autonomous agents over the network.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "message_type")]
pub enum AcpMessage {
    TaskDelegation {
        task_id: String,
        instruction: String,
        context: serde_json::Value,
    },
    TaskResult {
        task_id: String,
        status: AcpTaskStatus,
        output: serde_json::Value,
    },
    CapabilityDiscovery {
        agent_id: String,
        capabilities: Vec<String>,
    },
    Negotiation {
        session_id: String,
        proposal: serde_json::Value,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AcpTaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// Agent Identity and verifiable credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentIdentity {
    pub agent_id: String,
    pub public_key: String,
    pub signature: String,
    pub role: String,
}

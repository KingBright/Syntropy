use serde::{Deserialize, Serialize};

/// Represents a concept or entity understood by the Agent (e.g., "API Architecture", "User A")
#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
}

/// A chunk of conversation or document to be retrieved during GraphRAG
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryChunk {
    pub id: Option<String>,
    pub content: String,
    pub embedding: Vec<f32>,
    pub timestamp: i64,
}

/// An edge representing that a MemoryChunk "mentions" an Entity
#[derive(Debug, Serialize, Deserialize)]
pub struct Mentions {
    pub in_id: String,  // from MemoryChunk
    pub out_id: String, // to Entity
    pub confidence: f32,
}

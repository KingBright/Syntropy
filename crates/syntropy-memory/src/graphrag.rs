use crate::database::MemoryStore;
use crate::schema::{Entity, MemoryChunk, Mentions};
use serde::{Deserialize, Serialize};
use syntropy_core::error::{Result, SyntropyError};
use tracing::instrument;

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphRagResult {
    pub chunk_content: String,
    pub similarity_score: f32,
    pub mentioned_entities: Vec<String>,
}

pub struct GraphRagEngine {
    store: MemoryStore,
}

impl GraphRagEngine {
    pub fn new(store: MemoryStore) -> Self {
        Self { store }
    }

    /// Stores a memory chunk and links it to multiple entities via a "mentions" edge
    #[instrument(skip(self, chunk, entity_names))]
    pub async fn store_memory_with_entities(
        &self,
        chunk: MemoryChunk,
        entity_names: Vec<String>,
    ) -> Result<()> {
        let db = self.store.db();
        
        // 1. Create the chunk record
        let created: Option<MemoryChunk> = db.create("memory_chunk")
            .content(chunk)
            .await
            .map_err(|e| SyntropyError::Database(e.to_string()))?;
        
        let chunk_id = created.and_then(|c| c.id.clone())
            .ok_or_else(|| SyntropyError::Database("Failed to get created chunk ID".into()))?;

        // 2. Iterate and create entities if they don't exist, then link them
        for name in entity_names {
            // Upsert entity (assuming name is unique or we use it as ID for simplicity here)
            let entity_id = format!("entity:{}", name.replace(" ", "_").to_lowercase());
            
            let new_entity = Entity { 
                id: Some(entity_id.clone()), 
                name: name.clone(), 
                description: None 
            };
            let _entity: Option<Entity> = db.update(( "entity", entity_id.split(':').nth(1).unwrap() ))
                .merge(new_entity)
                .await
                .map_err(|e| SyntropyError::Database(e.to_string()))?;

            // Create the Edge "mentions"
            db.query("RELATE type::thing($in) -> mentions -> type::thing($out) SET confidence = 1.0;")
                .bind(("in", chunk_id.clone()))
                .bind(("out", entity_id.clone()))
                .await
                .map_err(|e| SyntropyError::Database(e.to_string()))?;
        }

        Ok(())
    }

    /// Retrieves relevant memory chunks utilizing vector search and graph traversal in a single SurrealQL query
    #[instrument(skip(self, query_embedding))]
    pub async fn retrieve_context(
        &self,
        query_embedding: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<GraphRagResult>> {
        let db = self.store.db();
        
        // SOTA GraphRAG logic:
        // Match chunk by vector similarity, then traverse ->mentions->entity to pull context
        let sql = "
            SELECT 
                content AS chunk_content, 
                vector::similarity::cosine(embedding, $embedding) AS similarity_score,
                ->mentions->entity.name AS mentioned_entities
            FROM memory_chunk 
            WHERE embedding <|4|> $embedding
            ORDER BY similarity_score DESC
            LIMIT $limit;
        ";

        let mut response = db.query(sql)
            .bind(("embedding", query_embedding))
            .bind(("limit", limit))
            .await
            .map_err(|e| SyntropyError::Database(e.to_string()))?;

        let results: Vec<GraphRagResult> = response.take(0)
            .map_err(|e| SyntropyError::Database(e.to_string()))?;

        Ok(results)
    }
}

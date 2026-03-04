use surrealdb::engine::any::{connect, Any};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use syntropy_core::config::DatabaseConfig;
use syntropy_core::error::{Result, SyntropyError};
use tracing::{info, instrument};

#[derive(Clone)]
pub struct MemoryStore {
    db: Surreal<Any>,
}

impl MemoryStore {
    #[instrument(skip(config))]
    pub async fn connect(config: &DatabaseConfig) -> Result<Self> {
        info!("Connecting to SurrealDB at {}", config.url);

        // Connect to the database backend (memory, rocksdb, or remote)
        let db = connect(&config.url)
            .await
            .map_err(|e| SyntropyError::Database(e.to_string()))?;

        // Authenticate as Root (In production, use specific user credentials)
        db.signin(Root {
            username: "root",
            password: "root_password", // TODO: Read from secure vault
        })
        .await
        .map_err(|e| SyntropyError::Database(e.to_string()))?;

        // Select the namespace and database
        db.use_ns(&config.namespace)
            .use_db(&config.database_name)
            .await
            .map_err(|e| SyntropyError::Database(e.to_string()))?;

        info!(
            "SurrealDB connection established targeting NS '{}' DB '{}'",
            config.namespace, config.database_name
        );

        Ok(Self { db })
    }

    /// Access the raw SurrealDB handle
    pub fn db(&self) -> &Surreal<Any> {
        &self.db
    }
}

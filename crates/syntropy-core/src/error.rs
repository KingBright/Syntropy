use thiserror::Error;

/// The central Error type for the Syntropy Framework.
#[derive(Error, Debug)]
pub enum SyntropyError {
    #[error("Configuration error: {0}")]
    Config(#[from] figment::Error),

    #[error("Database error: {0}")]
    Database(String), // We'll map surrealdb errors here later

    #[error("MCP Protocol error: {0}")]
    Mcp(String),

    #[error("Actor execution error: {0}")]
    Actor(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal system error: {0}")]
    Internal(#[from] anyhow::Error),
}

/// A specialized Result type for Syntropy operations.
pub type Result<T> = std::result::Result<T, SyntropyError>;

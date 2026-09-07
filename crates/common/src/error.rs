use thiserror::Error;

/// Unified application error system for Project PLATO.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;

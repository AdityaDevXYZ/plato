use thiserror::Error;

/// Central application error type.
/// Separates application orchestration failures from pure domain invariants.
#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Domain error: {0}")]
    Domain(String),

    #[error("Repository error: {0}")]
    Repository(String),

    #[error("Not found: {0}")]
    NotFound(String),
}

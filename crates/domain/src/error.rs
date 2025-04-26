use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Name must be at least 1 character long")]
    NameTooShort,
    #[error("External API error: {0}")]
    ExternalApiError(String),
    #[error("Database saving error: {0}")]
    PersistenceError(String),
    #[error("Record not found")]
    NotFound,
}

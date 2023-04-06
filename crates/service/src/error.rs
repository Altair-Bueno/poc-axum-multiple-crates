use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error(transparent)]
    DatabaseError(#[from] mongodb::error::Error),
    #[error("Couldn't find the requested resource")]
    NotFound,
}

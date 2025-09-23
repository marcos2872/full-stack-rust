use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("Database query error: {0}")]
    Query(#[from] sqlx::Error),

    #[error("Query error: {0}")]
    FailedQuery(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Hashing error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
}
use diesel::ConnectionError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, BudgieError>;

#[derive(Error, Debug)]
pub enum BudgieError {
    #[error("Database error: {0}")]
    DbError(#[from] diesel::result::Error),
    #[error("Database connection error: {0}")]
    ConnectionError(#[from] ConnectionError),
    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),
}

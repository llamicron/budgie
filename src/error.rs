use diesel::ConnectionError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, BudgieError>;

/// An internal error
#[derive(Error, Debug)]
pub enum BudgieError {
    #[error("Database error: {0}")]
    DbError(#[from] diesel::result::Error),
    #[error("Database connection error: {0}")]
    ConnectionError(#[from] ConnectionError),
    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
}

/// A user facing error
#[derive(Error, Debug)]
pub enum UserError {
    #[error("Error: {0}")]
    CustomError(String),
}

impl actix_web::error::ResponseError for BudgieError {}
impl actix_web::error::ResponseError for UserError {}

use thiserror::Error;

pub type Result<T> = std::result::Result<T, BudgieError>;

#[derive(Error, Debug)]
pub enum BudgieError {
    #[error("Database error: {0}")]
    DbError(#[from] diesel::result::Error),
}

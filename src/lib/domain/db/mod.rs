pub mod fields;

use sqlx;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Database error occurred {0}")]
    Database(#[from] sqlx::Error),
}

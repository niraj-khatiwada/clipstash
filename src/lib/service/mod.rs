pub mod clip;

use sqlx;
use thiserror;

use crate::domain::{clip::ClipError, db::DbError};

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Clip Error: {0}")]
    Clip(#[from] ClipError),
    #[error("Database Error: {0}")]
    Database(DbError),
    #[error("Not Found")]
    NotFound,
    #[error("Not sufficient permission {0}")]
    PermissionDenied(String),
    #[error("Internal Server Error")]
    InternalServer,
}

impl From<DbError> for ServiceError {
    fn from(error: DbError) -> Self {
        match error {
            DbError::Database(db_error) => match db_error {
                sqlx::Error::RowNotFound => Self::NotFound,
                err => Self::Database(DbError::Database(err)),
            },
        }
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(sqlx_error: sqlx::Error) -> Self {
        match sqlx_error {
            sqlx::Error::RowNotFound => Self::NotFound,
            other => Self::Database(DbError::Database(other)),
        }
    }
}

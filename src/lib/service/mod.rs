pub mod clip;

use actix_web::{
    http::{header, StatusCode},
    HttpResponse, ResponseError,
};
use serde::{Deserialize, Serialize};
use sqlx;
use thiserror;

use crate::domain::{clip::ClipError, db::DbError};

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Clip Error: {0}")]
    Clip(#[from] ClipError),
    #[error("Database Error: {0}")]
    Database(DbError),
    #[error("{0}")]
    NotFound(String),
    #[error("Not sufficient permission {0}")]
    PermissionDenied(String),
    #[error("Internal Server Error")]
    InternalServer,
}

impl From<DbError> for ServiceError {
    fn from(error: DbError) -> Self {
        match error {
            DbError::Database(db_error) => match db_error {
                sqlx::Error::RowNotFound => Self::NotFound(String::from("Resource not found.")),
                err => Self::Database(DbError::Database(err)),
            },
        }
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(sqlx_error: sqlx::Error) -> Self {
        match sqlx_error {
            sqlx::Error::RowNotFound => Self::NotFound(String::from("Resource not found.")),
            other => Self::Database(DbError::Database(other)),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Error {
    code: String,
    message: String,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        let err = Error {
            message: self.to_string(),
            code: self.status_code().to_string(),
        };
        HttpResponse::build(self.status_code())
            .insert_header(header::ContentType::json())
            .body(serde_json::to_string(&err).unwrap())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::NotFound(_) => StatusCode::NOT_FOUND,
            ServiceError::PermissionDenied(_) => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

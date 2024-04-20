pub mod field;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Clip {
    pub id: field::id::Id,
    pub short_code: field::short_code::ShortCode,
    pub title: field::title::Title,
    pub content: field::content::Content,
    pub password: field::password::Password,
    pub created_at: field::created_at::CreatedAt,
    pub expires_at: field::expires_at::ExpiresAt,
    pub no_of_hits: field::no_of_hits::NoOfHits,
}

#[derive(Debug, Error)]
pub enum ClipError {
    #[error("Invalid value for field {0}.")]
    InvalidValue(String),
    #[error("Empty value for field {0}")]
    EmptyValue(String),
    #[error("Invalid date for field {0}")]
    InvalidDate(String),
    #[error("Date parse error {0}")]
    InvalidParsedDate(#[from] chrono::ParseError),
    #[error("Invalid UUID {0}")]
    InvalidUUID(#[from] uuid::Error),
}

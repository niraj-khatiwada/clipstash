pub mod fields;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Clip {
    pub id: fields::id::Id,
    pub short_code: fields::short_code::ShortCode,
    pub title: fields::title::Title,
    pub content: fields::content::Content,
    pub password: fields::password::Password,
    pub created_at: fields::created_at::CreatedAt,
    pub expires_at: fields::expires_at::ExpiresAt,
    pub no_of_hits: fields::no_of_hits::NoOfHits,
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
    InvalidParsedDate(#[source] chrono::ParseError),
    #[error("Invalid UUID {0}")]
    InvalidUUID(#[source] uuid::Error),
    #[error("Invalid parsed UUID {0}")]
    InvalidParsedUUID(#[source] uuid::Error),
}

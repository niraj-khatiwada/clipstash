use std::str::FromStr;

use chrono::NaiveDateTime;
use sqlx;

use crate::domain::{
    clip::{
        fields::{self as clip_fields, short_code::ShortCode},
        Clip as DomainClip, ClipError,
    },
    datetime::DateTime,
};

#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub(in crate::db) id: String,
    pub(in crate::db) short_code: String,
    pub(in crate::db) title: Option<String>,
    pub(in crate::db) content: String,
    pub(in crate::db) password: Option<String>,
    pub(in crate::db) created_at: NaiveDateTime,
    pub(in crate::db) expires_at: Option<NaiveDateTime>,
    pub(in crate::db) no_of_hits: i64,
}

impl TryFrom<Clip> for DomainClip {
    type Error = ClipError;
    fn try_from(clip: Clip) -> Result<Self, Self::Error> {
        Ok(Self {
            id: clip_fields::id::Id::from_str(clip.id.as_str())?,
            short_code: clip_fields::short_code::ShortCode::from_str(clip.short_code.as_str())?,
            title: clip_fields::title::Title::new(clip.title)?,
            content: clip_fields::content::Content::new(&clip.content.as_str())?,
            password: clip_fields::password::Password::new(clip.password)?,
            created_at: clip_fields::created_at::CreatedAt::new(DateTime::from_naive(
                clip.created_at,
            )),
            expires_at: clip_fields::expires_at::ExpiresAt::new(
                clip.expires_at.map(DateTime::from_naive),
            ),
            no_of_hits: clip_fields::no_of_hits::NoOfHits::new(clip.no_of_hits),
        })
    }
}

pub struct GetClip {
    pub(in crate::db) short_code: String,
}

impl From<ShortCode> for GetClip {
    fn from(sc: ShortCode) -> Self {
        Self {
            short_code: sc.into_inner(),
        }
    }
}

impl From<String> for GetClip {
    fn from(sc: String) -> Self {
        Self { short_code: sc }
    }
}

pub struct InsertClip {
    pub short_code: String,
    pub title: Option<String>,
    pub content: String,
    pub password: Option<String>,
    pub expires_at: Option<NaiveDateTime>,
}

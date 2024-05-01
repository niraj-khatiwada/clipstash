use serde::{Deserialize, Serialize};

use crate::domain::clip::fields::{
    content::Content, expires_at::ExpiresAt, password::Password, short_code::ShortCode,
    title::Title,
};

#[derive(Serialize, Deserialize)]
pub struct UpdateClip {
    pub short_code: ShortCode,
    pub content: Content,
    pub title: Title,
    pub password: Password,
    pub expires_at: ExpiresAt,
}

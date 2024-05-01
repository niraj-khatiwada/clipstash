use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdateClip {
    pub content: Option<String>,
    pub title: Option<String>,
    pub password: Option<String>,
    pub expires_at: Option<NaiveDateTime>,
}

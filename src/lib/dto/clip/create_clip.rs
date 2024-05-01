use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateClip {
    pub short_code: String,
    pub content: String,
    pub title: Option<String>,
    pub password: Option<String>,
    pub expires_at: Option<String>, // Format should be ISO String without timezone. For eg: "2024-05-01T16:54:05.372"
}

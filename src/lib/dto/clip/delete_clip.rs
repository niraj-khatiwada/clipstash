use serde::{Deserialize, Serialize};

use crate::domain::clip::fields::short_code::ShortCode;

#[derive(Serialize, Deserialize)]
pub struct DeleteClip {
    pub short_code: String,
}

impl From<ShortCode> for DeleteClip {
    fn from(sc: ShortCode) -> Self {
        Self {
            short_code: sc.into_inner(),
        }
    }
}

impl From<String> for DeleteClip {
    fn from(string: String) -> Self {
        Self { short_code: string }
    }
}

impl From<&str> for DeleteClip {
    fn from(string: &str) -> Self {
        Self {
            short_code: String::from(string),
        }
    }
}

use crate::domain::clip::fields::short_code::ShortCode;

pub struct DeleteClip {
    pub short_code: ShortCode,
}

impl From<ShortCode> for DeleteClip {
    fn from(sc: ShortCode) -> Self {
        Self { short_code: sc }
    }
}

impl From<String> for DeleteClip {
    fn from(string: String) -> Self {
        Self {
            short_code: ShortCode::from(string.as_str()),
        }
    }
}

impl From<&str> for DeleteClip {
    fn from(string: &str) -> Self {
        Self {
            short_code: ShortCode::from(string),
        }
    }
}

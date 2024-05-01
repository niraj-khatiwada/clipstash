use crate::domain::clip::fields::{password::Password, short_code::ShortCode};

pub struct GetClip {
    pub short_code: ShortCode,
    pub password: Password,
}

impl GetClip {
    fn from_raw(short_code: &str) -> Self {
        Self {
            short_code: short_code.into(),
            password: Password::default(),
        }
    }
}

impl From<ShortCode> for GetClip {
    fn from(sc: ShortCode) -> Self {
        GetClip::from_raw(sc.into_inner().as_str())
    }
}

impl From<&str> for GetClip {
    fn from(ss: &str) -> Self {
        GetClip::from_raw(ss)
    }
}

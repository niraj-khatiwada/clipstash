use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use crate::domain::clip::ClipError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortCode(String);

impl ShortCode {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4().to_string();
        Self(uuid)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Default for ShortCode {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ShortCode> for String {
    fn from(value: ShortCode) -> Self {
        value.0
    }
}

impl From<String> for ShortCode {
    fn from(val: String) -> Self {
        Self(val)
    }
}

impl From<&str> for ShortCode {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl FromStr for ShortCode {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

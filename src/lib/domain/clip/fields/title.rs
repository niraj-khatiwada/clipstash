use std::str::FromStr;

use super::super::ClipError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Title(Option<String>);

impl Title {
    pub fn new<T: Into<Option<String>>>(title: T) -> Result<Self, ClipError> {
        let title: Option<String> = title.into();

        if let Some(t) = title {
            if t.len() > 0 {
                return Ok(Self(Some(t)));
            } else {
                // When the title is given, it must not be empty though
                return Err(ClipError::EmptyValue(String::from("title")));
            }
        }
        return Ok(Self(None));
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

impl Default for Title {
    fn default() -> Self {
        return Self(None);
    }
}

impl FromStr for Title {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Self(Some(String::from(s))));
    }
}

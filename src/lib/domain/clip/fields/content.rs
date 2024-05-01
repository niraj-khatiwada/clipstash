use super::super::ClipError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content(String);

impl Content {
    pub fn new(content: &str) -> Result<Self, ClipError> {
        if content.trim().len() > 0 {
            Ok(Content(String::from(content)))
        } else {
            Err(ClipError::EmptyValue(String::from("content")))
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

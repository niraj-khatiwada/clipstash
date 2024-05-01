use std::str::FromStr;

use super::super::ClipError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Password(Option<String>); // Password is optional

impl Password {
    pub fn new<T: Into<Option<String>>>(password: T) -> Result<Self, ClipError> {
        let password: Option<String> = password.into();

        if let Some(value) = password {
            if value.len() > 0 {
                return Ok(Self(Some(value)));
            } else {
                // When the password is given, it must not be empty though
                return Err(ClipError::EmptyValue(String::from("password")));
            }
        }
        return Ok(Self(None));
    }

    pub fn into_inner(self) -> Option<String> {
        return self.0;
    }

    pub fn has_password(&self) -> bool {
        self.0.is_some()
    }

    pub fn are_passwords_equal(&self, new_password: &str) -> bool {
        if self.0.is_some() {
            if let Some(password) = &self.0 {
                return password == new_password;
            }
        }
        false
    }
}

impl Default for Password {
    fn default() -> Self {
        Self(None)
    }
}

// Easily converts &str (string slice) to Password type
// FromStr trait should return  Result<Self, Self::Err>
impl FromStr for Password {
    type Err = ClipError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        return Self::new(String::from(string));
    }
}

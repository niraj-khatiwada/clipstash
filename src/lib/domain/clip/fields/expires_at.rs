use crate::domain::{clip::ClipError, datetime::DateTime};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpiresAt(Option<DateTime>);

impl ExpiresAt {
    pub fn new<T: Into<Option<DateTime>>>(value: T) -> Self {
        let value: Option<DateTime> = value.into();
        Self(value)
    }

    pub fn into_inner(self) -> Option<DateTime> {
        return self.0;
    }
}

impl Default for ExpiresAt {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for ExpiresAt {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match DateTime::from_str(s) {
            Ok(dt) => Ok(Self(Some(dt))),
            Err(error) => Err(error),
        }
    }
}

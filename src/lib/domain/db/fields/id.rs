use derive_more::From;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use crate::domain::clip::ClipError;

#[derive(Debug, Serialize, Deserialize, From)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> Self {
        return Uuid::new_v4().into();
    }

    pub fn into_inner(self) -> Uuid {
        return self.0;
    }

    pub fn nil() -> Self {
        return Self(Uuid::nil());
    }
}

impl Default for DbId {
    fn default() -> Self {
        return Self::new();
    }
}

impl FromStr for DbId {
    type Err = ClipError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Uuid::try_parse(s) {
            Ok(uuid) => Ok(Self(uuid)),
            Err(err) => Err(ClipError::InvalidParsedUUID(err)),
        }
    }
}

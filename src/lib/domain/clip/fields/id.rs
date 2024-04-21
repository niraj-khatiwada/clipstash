use crate::domain::clip::ClipError;
use crate::domain::db::fields::id::DbId;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Id(DbId);

impl Id {
    pub fn new() -> Self {
        return Self(Uuid::new_v4().into());
    }

    pub fn into_inner(self) -> DbId {
        return self.0;
    }
}

impl From<DbId> for Id {
    fn from(db_id: DbId) -> Self {
        return Self(db_id);
    }
}

impl Default for Id {
    fn default() -> Self {
        return Self(DbId::nil());
    }
}

impl FromStr for Id {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Uuid::try_parse(s) {
            Ok(uuid) => Ok(Self(uuid.into())),
            Err(err) => Err(ClipError::InvalidParsedUUID(err)),
        }
    }
}

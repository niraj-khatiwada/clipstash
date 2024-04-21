use crate::domain::datetime::DateTime;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Constructor)]
pub struct CreatedAt(DateTime);

impl CreatedAt {
    pub fn to_inner(self) -> DateTime {
        self.0
    }
}

impl Default for CreatedAt {
    fn default() -> Self {
        Self(DateTime::now())
    }
}

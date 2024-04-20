use serde::{Deserialize, Serialize};

use crate::domain::datetime::DateTime;

#[derive(Debug, Serialize, Deserialize)]
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

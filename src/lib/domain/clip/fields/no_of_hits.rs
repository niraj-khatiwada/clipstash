use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Constructor)]
pub struct NoOfHits(i64);

impl NoOfHits {
    // The Constructor derive above does this same thing below. Since, it's used a lot, we can use Constructor derive instead.
    // pub fn new(value: i64) -> Self {
    //     Self(value)
    // }
    pub fn into_inner(self) -> i64 {
        self.0
    }
}

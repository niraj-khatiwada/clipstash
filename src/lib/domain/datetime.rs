use std::str::FromStr;

use chrono::{DateTime as ChronoDateTime, NaiveDateTime, Utc};
use derive_more::From;
use serde::{Deserialize, Serialize};

use super::clip::ClipError;

type DateTimeUTC = ChronoDateTime<Utc>;

#[derive(Debug, Serialize, Deserialize, From, Clone)]
pub struct DateTime(DateTimeUTC);

impl DateTime {
    pub fn now() -> Self {
        return Self(Utc::now());
    }

    pub fn into_inner(self) -> DateTimeUTC {
        self.0
    }

    pub fn timestamp(&self) -> i64 {
        self.0.timestamp()
    }

    pub fn to_naive(&self) -> NaiveDateTime {
        return self.0.naive_utc();
    }

    pub fn from_naive(naive_datetime: NaiveDateTime) -> Self {
        return Self(naive_datetime.and_utc());
    }
}

impl FromStr for DateTime {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // ChronoDateTime::from_str(s) must require the date format be like this  "2024-04-20T00:00:00Z"
        // So, we are appending the time info if not present
        let string = if s.len() == 10 {
            format!("{}T00:00:00Z", s)
        } else {
            String::from(s)
        };

        match ChronoDateTime::from_str(&string) {
            Ok(value) => Ok(Self(value)),
            Err(err) => Err(ClipError::InvalidParsedDate(err)),
        }
    }
}

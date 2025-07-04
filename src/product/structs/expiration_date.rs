use std::fmt;
use chrono::{DateTime, Local};

pub struct ExpirationDate(DateTime<Local>);
impl ExpirationDate {
    pub fn new(datetime: DateTime<Local>) -> Self {
        Self(datetime)
    }

    pub fn now() -> Self {
        Self(Local::now())
    }

    pub fn get_expiry_date(&self) -> DateTime<Local> {
        self.0
    }

    pub fn is_expired(&self) -> bool {
        self.0.timestamp() - Local::now().timestamp() <= 0
    }
}

impl fmt::Display for ExpirationDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%A, %Y-%m-%d %H:%M:%S"))
    }
}

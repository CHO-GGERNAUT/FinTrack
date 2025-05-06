use chrono::{NaiveDate, ParseError, Utc};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExpirationDate {
    year: u32,
    month: u8,
}

#[derive(Debug, Error)]
pub enum ExpirationDateError {
    #[error("Invalid format: expected MM/YY or MM/YYYY")]
    InvalidFormat,
    #[error("Invalid month: {0} (must be 1-12)")]
    InvalidMonth(u8),
    #[error("Invalid year: {0} (must be 2000-2100)")]
    InvalidYear(u32),
    #[error("Date parsing failed: {0}")]
    DateParse(#[from] ParseError),
}

impl ExpirationDate {
    pub fn new(year: u32, month: u8) -> Result<Self, ExpirationDateError> {
        if !(1..=12).contains(&month) {
            return Err(ExpirationDateError::InvalidMonth(month));
        }
        if !(2000..=2100).contains(&year) {
            return Err(ExpirationDateError::InvalidYear(year));
        }

        Ok(Self { year, month })
    }

    pub fn year(&self) -> u32 {
        self.year
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn is_expired(&self) -> bool {
        let last_day = Self::last_day_of_month(self.year, self.month);
        last_day < Utc::now().date_naive()
    }

    fn last_day_of_month(year: u32, month: u8) -> NaiveDate {
        let (next_year, next_month) = if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };
        NaiveDate::from_ymd_opt(next_year as i32, next_month as u32, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
    }
}

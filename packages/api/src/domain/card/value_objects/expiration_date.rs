use chrono::{NaiveDate, ParseError, Utc};
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExpirationDate {
    year: i32,
    month: u32,
}

#[derive(Debug, Error)]
pub enum ExpirationDateError {
    #[error("Invalid format: expected MM/YY or MM/YYYY")]
    InvalidFormat,
    #[error("Invalid month: {0} (must be 1-12)")]
    InvalidMonth(u32),
    #[error("Invalid year: {0} (must be 2000-2100)")]
    InvalidYear(i32),
    #[error("Date parsing failed: {0}")]
    DateParse(#[from] ParseError),
}

impl ExpirationDate {
    pub fn new(month: u32, year: i32) -> Result<Self, ExpirationDateError> {
        if !(1..=12).contains(&month) {
            return Err(ExpirationDateError::InvalidMonth(month));
        }
        if !(2000..=2100).contains(&year) {
            return Err(ExpirationDateError::InvalidYear(year));
        }

        Ok(Self { year, month })
    }

    pub fn from_str(input: &str) -> Result<Self, ExpirationDateError> {
        let parts: Vec<_> = input.trim().split('/').collect();
        if parts.len() != 2 {
            return Err(ExpirationDateError::InvalidFormat);
        }

        let month = parts[0]
            .parse::<u32>()
            .map_err(|_| ExpirationDateError::InvalidFormat)?;
        let year = match parts[1].len() {
            2 => {
                2000 + parts[1]
                    .parse::<i32>()
                    .map_err(|_| ExpirationDateError::InvalidFormat)?
            }
            4 => parts[1]
                .parse::<i32>()
                .map_err(|_| ExpirationDateError::InvalidFormat)?,
            _ => return Err(ExpirationDateError::InvalidFormat),
        };

        Self::new(month, year)
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn month(&self) -> u32 {
        self.month
    }

    pub fn format_mm_yy(&self) -> String {
        format!("{:02}/{:02}", self.month, self.year % 100)
    }

    pub fn format_mm_yyyy(&self) -> String {
        format!("{:02}/{}", self.month, self.year)
    }

    pub fn is_expired(&self) -> bool {
        let last_day = Self::last_day_of_month(self.year, self.month);
        last_day < Utc::now().date_naive()
    }

    fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
        let (next_year, next_month) = if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };
        NaiveDate::from_ymd_opt(next_year, next_month, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
    }
}

impl TryFrom<&str> for ExpirationDate {
    type Error = ExpirationDateError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

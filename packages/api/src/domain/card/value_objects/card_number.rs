use std::convert::TryFrom;

use crate::domain::shared::errors::DomainValidationRuleError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CardNumber(String); // Store as String, potentially encrypted

impl CardNumber {
    fn is_valid(number: &str) -> bool {
        let num = number.replace(" ", "").replace("-", "");
        if num.len() < 13 || num.len() > 19 || !num.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }

        luhn_check(&num)
    }

    pub fn new(number: String) -> Result<Self, DomainValidationRuleError> {
        if number.trim().is_empty() {
            Err(DomainValidationRuleError::InvalidCardNumberFormat(
                "Card number cannot be empty".to_string(),
            ))
        } else if !Self::is_valid(&number) {
            Err(DomainValidationRuleError::InvalidCardNumberFormat(
                "Card number is invalid".to_string(),
            ))
        } else {
            Ok(CardNumber(number))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn last_four(&self) -> String {
        if self.0.len() >= 4 {
            self.0.chars().skip(self.0.len() - 4).collect()
        } else {
            self.0.clone()
        }
    }
}

fn luhn_check(number: &str) -> bool {
    let sum = number
        .chars()
        .rev()
        .enumerate()
        .filter_map(|(i, c)| c.to_digit(10).map(|d| (i, d)))
        .map(|(i, d)| {
            if i % 2 == 1 {
                let doubled = d * 2;
                if doubled > 9 { doubled - 9 } else { doubled }
            } else {
                d
            }
        })
        .sum::<u32>();
    sum % 10 == 0
}

impl TryFrom<String> for CardNumber {
    type Error = DomainValidationRuleError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        CardNumber::new(value)
    }
}

impl From<CardNumber> for String {
    fn from(value: CardNumber) -> Self {
        value.0
    }
}

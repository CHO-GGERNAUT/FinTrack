use crate::domain::shared::errors::DomainValidationRuleError;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(value: String) -> Result<Self, DomainValidationRuleError> {
        let digits_only = value.chars().all(|c| c.is_ascii_digit());

        if !digits_only {
            return Err(DomainValidationRuleError::InvalidPhoneNumberFormat(
                "Contains non-digit characters".into(),
            ));
        }

        if value.len() < 9 || value.len() > 15 {
            return Err(DomainValidationRuleError::InvalidPhoneNumberFormat(
                "Invalid Length".into(),
            ));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PhoneNumber {
    type Error = DomainValidationRuleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

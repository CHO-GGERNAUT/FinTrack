use super::super::UserError;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(value: &str) -> Result<Self, UserError> {
        let digits_only = value.chars().all(|c| c.is_ascii_digit());

        if !digits_only {
            return Err(UserError::InvalidPhoneNumber(
                "Contains non-digit characters".into(),
            ));
        }

        if value.len() < 9 || value.len() > 15 {
            return Err(UserError::InvalidPhoneNumber("Invalid Length".into()));
        }

        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

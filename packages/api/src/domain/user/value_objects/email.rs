use validator::ValidateEmail;

use super::super::errors::UserError;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn new(value: &str) -> Result<Self, UserError> {
        if value.validate_email() {
            Ok(Self(value.to_owned()))
        } else {
            Err(UserError::InvalidEmail)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

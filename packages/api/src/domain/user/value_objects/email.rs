use validator::ValidateEmail;

use super::super::UserError;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn new(value: &str) -> Result<Self, UserError> {
        match value.validate_email() {
            Ok(_) => Ok(Self(value.to_owned())),
            Err(_) => Err(UserError::InvalidEmail),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

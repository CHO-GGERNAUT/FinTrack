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

    pub fn from_persistent(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

use validator::ValidateEmail;

use crate::domain::shared::errors::DomainValidationRuleError;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn new(value: String) -> Result<Self, DomainValidationRuleError> {
        if value.validate_email() {
            Ok(Self(value))
        } else {
            Err(DomainValidationRuleError::InvalidEmailFormat(value))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Email {
    type Error = DomainValidationRuleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Email::new(value)
    }
}
impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

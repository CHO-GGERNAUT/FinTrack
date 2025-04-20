pub mod account;
pub mod card;
pub mod user;
pub type Result<T> = std::result::Result<T, DomainError>;

pub enum DomainError {
    CardError(card::CardError),
    UserError(user::UserError),
    AccountError(account::AccountError),
    CommonError(String),
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::CardError(err) => write!(f, "Card error: {:?}", err),
            DomainError::UserError(err) => write!(f, "User error: {:?}", err),
            DomainError::AccountError(err) => write!(f, "Account error: {:?}", err),
            DomainError::CommonError(msg) => write!(f, "Domain error: {}", msg),
        }
    }
}

impl From<card::CardError> for DomainError {
    fn from(err: card::CardError) -> Self {
        DomainError::CardError(err)
    }
}

impl From<user::UserError> for DomainError {
    fn from(err: user::UserError) -> Self {
        DomainError::UserError(err)
    }
}

impl From<account::AccountError> for DomainError {
    fn from(err: account::AccountError) -> Self {
        DomainError::AccountError(err)
    }
}

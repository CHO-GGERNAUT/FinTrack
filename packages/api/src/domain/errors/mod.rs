mod account;
pub use account::*;

mod card;
pub use card::*;

mod user;
pub use user::*;

mod uow;
pub use uow::*;

mod transaction;
pub use transaction::*;

mod merchant;
pub use merchant::*;

use thiserror::Error;
// domain/errors.rs
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("User error: {0}")]
    UserError(#[from] UserError),

    #[error("Account error: {0}")]
    AccountError(#[from] AccountError),

    #[error("Card error: {0}")]
    CardError(#[from] CardError),

    #[error("Transaction error: {0}")]
    TransactionError(#[from] TransactionError),

    #[error("Merchant error: {0}")]
    MerchantError(#[from] MerchantError),

    #[error("Unknown domain error")]
    Unknown {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

// impl From<AccountError> for DomainError {
//     fn from(e: AccountError) -> Self {
//         DomainError::AccountError(e)
//     }
// }
// impl From<UserError> for DomainError {
//     fn from(e: UserError) -> Self {
//         DomainError::UserError(e)
//     }
// }

// impl From<CardError> for DomainError {
//     fn from(e: CardError) -> Self {
//         DomainError::CardError(e)
//     }
// }
impl From<UowError> for DomainError {
    fn from(e: UowError) -> Self {
        DomainError::Unknown {
            source: Box::new(e),
        }
    }
}

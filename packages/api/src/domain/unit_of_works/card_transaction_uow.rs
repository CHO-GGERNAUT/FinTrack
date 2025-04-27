use async_trait::async_trait;

use crate::domain::repositories::{
    AccountRepository, CardTransactionRepository, MerchantRepository,
};

use super::UnitOfWork;

#[async_trait]
pub trait CardTransactionUnitOfWork: UnitOfWork {
    type AccountRepo<'a>: AccountRepository + Send
    where
        Self: 'a;
    type CardTransactionRepo<'a>: CardTransactionRepository + Send
    where
        Self: 'a;

    type MerchantRepo<'a>: MerchantRepository + Send
    where
        Self: 'a;

    fn transaction_repo(&mut self) -> Self::CardTransactionRepo<'_>;
    fn account_repo(&mut self) -> Self::AccountRepo<'_>;
    fn merchant_repo(&mut self) -> Self::MerchantRepo<'_>;
}

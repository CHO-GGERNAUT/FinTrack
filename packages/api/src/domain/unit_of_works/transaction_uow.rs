use async_trait::async_trait;

use crate::domain::repositories::{AccountRepository, TransactionRepository};

use super::UnitOfWork;

#[async_trait]
pub trait TransactionUnitOfWork: UnitOfWork {
    type AccountRepo<'a>: AccountRepository + Send
    where
        Self: 'a;
    type TransactionRepo<'a>: TransactionRepository + Send
    where
        Self: 'a;

    fn transaction_repo(&mut self) -> Self::TransactionRepo<'_>;
    fn account_repo(&mut self) -> Self::AccountRepo<'_>;
}

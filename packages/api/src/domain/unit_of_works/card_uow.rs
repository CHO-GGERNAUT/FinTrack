use async_trait::async_trait;

use crate::domain::repositories::{AccountRepository, CardRepository};

use super::UnitOfWork;

#[async_trait]
pub trait CardUnitOfWork: UnitOfWork {
    type AccountRepo<'a>: AccountRepository + Send
    where
        Self: 'a;
    type CardRepo<'a>: CardRepository + Send
    where
        Self: 'a;

    fn account_repo(&mut self) -> Self::AccountRepo<'_>;
    fn card_repo(&mut self) -> Self::CardRepo<'_>;
}

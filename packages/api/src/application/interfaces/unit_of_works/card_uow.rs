use crate::domain::card::repository::CardRepository;

use super::UnitOfWork;

#[async_trait::async_trait]
pub trait CardUnitOfWork: UnitOfWork {
    type CardRepo<'a>: CardRepository
    where
        Self: 'a;

    fn card_repository(&mut self) -> Self::CardRepo<'_>;
}

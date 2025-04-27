use uuid::Uuid;

use crate::{
    application::errors::ApplicationError,
    domain::{entities::Card, repositories::CardRepository, unit_of_works::CardUnitOfWork},
};

#[derive(Clone)]
pub struct FindByIdUsecase<U: CardUnitOfWork> {
    pub uow: U,
}

impl<U: CardUnitOfWork> FindByIdUsecase<U> {
    pub fn new(uow: U) -> Self {
        Self { uow }
    }

    pub async fn execute(
        mut self,
        account_id: Uuid,
        user_id: Uuid,
    ) -> Result<Card, ApplicationError> {
        let card = self.uow.card_repo().find_by_id(account_id).await?;
        if card.account.owner_id != user_id {
            return Err(ApplicationError::ValidationError(
                "Unauthorized access".to_string(),
            ));
        };
        Ok(card)
    }
}

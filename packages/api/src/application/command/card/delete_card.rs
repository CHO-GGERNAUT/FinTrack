use crate::{
    application::{
        dto::{DeleteCardInput, DeleteCardOutput},
        errors::ApplicationError,
    },
    domain::{
        repositories::{AccountRepository, CardRepository},
        unit_of_works::CardUnitOfWork,
    },
};

pub struct DeleteCardUsecase<U: CardUnitOfWork> {
    pub uow: U,
}

impl<U: CardUnitOfWork> DeleteCardUsecase<U> {
    pub fn new(uow: U) -> Self {
        Self { uow }
    }
    pub async fn execute(
        mut self,
        input: DeleteCardInput,
    ) -> Result<DeleteCardOutput, ApplicationError> {
        {
            let mut account_repo = self.uow.account_repo();
            let account = account_repo.find_by_id(input.account_id).await?;
            if account.owner_id != input.owner_id {
                return Err(ApplicationError::ValidationError(
                    "You are not the owner of this account".to_string(),
                ));
            }
        }
        {
            let mut card_repo = self.uow.card_repo();
            card_repo.delete(input.account_id).await?;
        }
        {
            let mut account_repo = self.uow.account_repo();
            account_repo.delete(input.account_id).await?;
        }

        self.uow.commit().await?;

        Ok(DeleteCardOutput {
            account_id: input.account_id,
        })
    }
}

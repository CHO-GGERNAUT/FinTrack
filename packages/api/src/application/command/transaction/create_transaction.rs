use crate::{
    application::{
        dto::{CreateTransactionInput, CreateTransactionOutput},
        errors::ApplicationError,
    },
    domain::{
        entities::Transaction,
        repositories::{AccountRepository, TransactionRepository},
        unit_of_works::TransactionUnitOfWork,
    },
};
use chrono::Utc;
use uuid::Uuid;

pub struct CreateTransactionUsecase<U: TransactionUnitOfWork> {
    pub uow: U,
}

impl<U: TransactionUnitOfWork> CreateTransactionUsecase<U> {
    pub fn new(uow: U) -> Self {
        Self { uow }
    }
    pub async fn execute(
        mut self,
        input: CreateTransactionInput,
    ) -> Result<CreateTransactionOutput, ApplicationError> {
        let now = Utc::now();
        let transaction_id = Uuid::new_v4();

        let _ = self.uow.account_repo().find_by_id(input.account_id).await?;

        let transaction = Transaction {
            id: transaction_id,
            account_id: input.account_id,
            category_id: None,
            amount: input.amount,

            created_at: now,
            updated_at: now,
            deleted_at: None,
            memo: input.memo,
            approved_at: input.approved_at,
            transaction_type: input.transaction_type,
        };
        self.uow.transaction_repo().create(&transaction).await?;

        self.uow.commit().await?;

        Ok(CreateTransactionOutput { id: transaction_id })
    }
}

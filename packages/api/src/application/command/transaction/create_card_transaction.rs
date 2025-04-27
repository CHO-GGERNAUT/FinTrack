use crate::{
    application::{
        dto::{CreateCardTransactionInput, CreateCardTransactionOutput, MerchantInput},
        errors::ApplicationError,
    },
    domain::{
        entities::{CardTransaction, Merchant},
        repositories::{AccountRepository, CardTransactionRepository, MerchantRepository},
        unit_of_works::CardTransactionUnitOfWork,
    },
};
use chrono::Utc;
use uuid::Uuid;

pub struct CreateCardTransactionUsecase<U: CardTransactionUnitOfWork> {
    pub uow: U,
}

impl<U: CardTransactionUnitOfWork> CreateCardTransactionUsecase<U> {
    pub fn new(uow: U) -> Self {
        Self { uow }
    }
    pub async fn execute(
        mut self,
        input: CreateCardTransactionInput,
    ) -> Result<CreateCardTransactionOutput, ApplicationError> {
        let now = Utc::now();
        let transaction_id = Uuid::new_v4();
        let account = self.uow.account_repo().find_by_id(input.card_id).await?;
        if account.owner_id != input.user_id {
            return Err(ApplicationError::RepositoryError(
                "Account not found".to_string(),
            ));
        }
        let merchant_id = match input.merchant {
            MerchantInput::ById(merchant_id) => merchant_id,
            MerchantInput::ByInfo(merchant) => {
                let merchant = Merchant {
                    id: Uuid::new_v4(),
                    name: merchant.name,
                    biz_number: merchant.biz_number,
                    address: merchant.address,
                    phone: merchant.phone,
                    created_at: now,
                    updated_at: now,
                    deleted_at: None,
                };
                let res = self.uow.merchant_repo().create(&merchant).await?;
                res.id
            }
        };

        let transaction = CardTransaction {
            id: transaction_id,
            account_id: account.id,
            user_id: input.user_id,
            merchant_id,
            category_id: None,
            amount: input.amount,

            created_at: now,
            updated_at: now,
            deleted_at: None,
            memo: input.memo,
            approved_at: input.approved_at,
            transaction_type: input.transaction_type,
            installment_months: input.installment_months.unwrap_or_default(),
        };
        self.uow.transaction_repo().create(&transaction).await?;

        self.uow.commit().await?;

        Ok(CreateCardTransactionOutput {
            transaction_id,
            merchant_id,
        })
    }
}

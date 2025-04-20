use crate::{
    application::{
        Result,
        dto::{CreateCardInput, CreateCardOutput},
    },
    domain::{
        entities::{Account, Card},
        enums::AccountType,
        repositories::{AccountRepository, CardRepository},
        unit_of_works::CardUnitOfWork,
    },
};
use chrono::Utc;
use uuid::Uuid;

pub struct CreateCardUsecase<U: CardUnitOfWork> {
    pub uow: U,
}

impl<U: CardUnitOfWork> CreateCardUsecase<U> {
    pub fn new(uow: U) -> Self {
        Self { uow }
    }
    pub async fn execute(mut self, input: CreateCardInput) -> Result<CreateCardOutput> {
        let now = Utc::now();
        let account_id = Uuid::new_v4();

        let account = Account {
            id: account_id,
            owner_id: input.owner_id,
            created_at: now,
            updated_at: now,
            deleted_at: None,
            account_type: AccountType::Card,
        };
        self.uow.account_repo().create(&account).await?;

        let card = Card {
            account_id,
            card_number_last4: input.card_number_last4.clone(),
            encrypted_card_number: input.encrypted_card_number.clone(),
            issued_at: input.issued_at,
            expires_at: input.expires_at,
            billing_day: input.billing_day,
            brand: input.brand,
            issuer: input.issuer,
            card_type: input.card_type,
            created_at: now,
            updated_at: now,
            deleted_at: None,
            name: input.name,
            memo: input.memo,
        };
        self.uow.card_repo().create(&card).await?;

        self.uow.commit().await?;

        Ok(CreateCardOutput { account_id })
    }
}

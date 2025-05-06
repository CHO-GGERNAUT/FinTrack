use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    application::{errors::ApplicationError, interfaces::unit_of_works::CardUnitOfWork},
    domain::{
        card::{
            entities::Card,
            repository::CardRepository,
            value_objects::{CardBrand, CardIssuer, CardNumber, CardType, ExpirationDate},
        },
        user::value_objects::UserId,
    },
};

pub struct IssueCardCommand {
    pub user_id: Uuid,
    pub card_number: String,
    pub expiration_date: (u32, u8),
    pub issuance_date: NaiveDate,
    pub card_type: String,
    pub card_brand: String,
    pub card_issuer: String,
    pub name: Option<String>,
}

pub struct IssueCardResult {
    pub card_id: Uuid,
    pub created_at: String,
}

pub struct IssueCardHandler<U: CardUnitOfWork> {
    uow: U,
}

impl<U: CardUnitOfWork> IssueCardHandler<U> {
    pub fn new(uow: U) -> Self {
        Self { uow }
    }

    pub async fn execute(
        mut self,
        command: IssueCardCommand,
    ) -> Result<IssueCardResult, ApplicationError> {
        let card_number =
            CardNumber::new(command.card_number).map_err(|e| ApplicationError::validation(e))?;

        let expiration_date =
            ExpirationDate::new(command.expiration_date.0, command.expiration_date.1)
                .map_err(|e| ApplicationError::validation(e))?;

        let card_type = CardType::try_from(command.card_type.as_str())
            .map_err(|e| ApplicationError::validation(e))?;
        let card_brand = CardBrand::try_from(command.card_brand.as_str())
            .map_err(|e| ApplicationError::validation(e))?;
        let card_issuer = CardIssuer::try_from(command.card_issuer.as_str())
            .map_err(|e| ApplicationError::validation(e))?;

        let user_id: UserId = command.user_id.into();
        let card = Card::issue(
            user_id,
            card_number,
            expiration_date,
            command.issuance_date,
            card_type,
            card_brand,
            card_issuer,
            command.name,
        )
        .map_err(|e| ApplicationError::validation(e))?;
        let res = self.uow.card_repository().create(card).await?;

        self.uow.commit().await?;

        Ok(IssueCardResult {
            card_id: (*res.id()).into(),
            created_at: res.audit_info().created_at().to_string(),
        })
    }
}

use crate::{
    domain::{
        entities::Card,
        errors::{CardError, DomainError},
        repositories::CardRepository,
    },
    infrastructure::db::schema::{CardBrandDb, CardIssuerDb, CardRow, CardTypeDb},
};
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub struct CardRepositoryPostgres<'a> {
    tx: &'a mut Transaction<'static, Postgres>,
}

impl<'a> CardRepositoryPostgres<'a> {
    pub fn new(tx: &'a mut Transaction<'static, Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl<'a> CardRepository for CardRepositoryPostgres<'a> {
    async fn create(&mut self, card: &Card) -> Result<Card, DomainError> {
        let tx = self.tx.as_mut();

        let row = sqlx::query_as!(
            CardRow,
            r#"
                INSERT INTO card (
                    account_id,
                    card_number_last4,
                    encrypted_card_number,
                    issued_at,
                    expires_at,
                    billing_day,
                    brand,
                    issuer,
                    card_type,
                    name,
                    memo
                )
                VALUES (
                    $1, $2, $3, $4, $5, $6,
                    $7::card_brand, $8::card_issuer, $9::card_type,
                    $10, $11
                )
                RETURNING
                    account_id,
                    created_at,
                    updated_at,
                    deleted_at,
                    card_number_last4,
                    encrypted_card_number,
                    issued_at,
                    expires_at,
                    billing_day,
                    brand as "brand:CardBrandDb",
                    issuer as "issuer:CardIssuerDb",
                    card_type as "card_type:CardTypeDb",
                    name,
                    memo
            "#,
            card.account_id,
            card.card_number_last4,
            card.encrypted_card_number,
            card.issued_at,
            card.expires_at,
            card.billing_day,
            CardBrandDb::from(card.brand) as CardBrandDb,
            CardIssuerDb::from(card.issuer) as CardIssuerDb,
            CardTypeDb::from(card.card_type) as CardTypeDb,
            card.name,
            card.memo,
        )
        .fetch_one(tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save card: {}", e);
            CardError::Duplicate
        })?;

        Ok(row.into())
    }

    async fn delete(&mut self, account_id: Uuid) -> Result<(), DomainError> {
        let tx = self.tx.as_mut();

        sqlx::query!(r#"DELETE FROM card WHERE account_id = $1"#, account_id)
            .execute(tx)
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete card: {}", e);
                CardError::NotFound
            })?;

        Ok(())
    }
}

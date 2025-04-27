use crate::{
    domain::{
        entities::{Account, Card},
        errors::{CardError, DomainError},
        repositories::CardRepository,
    },
    infrastructure::db::schema::{
        AccountRow, AccountTypeDb, CardBrandDb, CardIssuerDb, CardRow, CardTypeDb,
    },
};
use async_trait::async_trait;
use sqlx::{Postgres, Transaction, query};
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
        let account_row = sqlx::query_as!(
            AccountRow,
            r#"
            INSERT INTO accounts (id, owner_id, account_type)
            VALUES ($1, $2, $3)
            RETURNING 
                id,
                owner_id,
                created_at,
                updated_at,
                deleted_at,
                account_type as "account_type:AccountTypeDb"
            "#,
            card.account.id,
            card.account.owner_id,
            AccountTypeDb::Card as AccountTypeDb,
        )
        .fetch_one(self.tx.as_mut())
        .await
        .map_err(|e| {
            tracing::error!("DB error: {}", e);
            CardError::Duplicate
        })?;

        let card_row = sqlx::query_as!(
            CardRow,
            r#"
                INSERT INTO cards (
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
            account_row.id,
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
        .fetch_one(self.tx.as_mut())
        .await
        .map_err(|e| {
            tracing::error!("Failed to save card: {}", e);
            CardError::Duplicate
        })?;

        Ok((card_row, account_row).into())
    }

    async fn delete(&mut self, account_id: Uuid) -> Result<(), DomainError> {
        let tx = self.tx.as_mut();

        sqlx::query!(r#"DELETE FROM cards WHERE account_id = $1"#, account_id)
            .execute(tx)
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete card: {}", e);
                CardError::NotFound
            })?;

        Ok(())
    }

    async fn find_by_id(&mut self, account_id: Uuid) -> Result<Card, DomainError> {
        let tx = self.tx.as_mut();

        let row = query!(
            r#"
            SELECT 
                accounts.id AS account_id,
                accounts.owner_id,
                accounts.created_at,
                accounts.updated_at,
                accounts.deleted_at,
    
                cards.card_number_last4,
                cards.encrypted_card_number,
                cards.issued_at,
                cards.expires_at,
                cards.billing_day,
                cards.brand as "brand:CardBrandDb",
                cards.issuer as "issuer:CardIssuerDb",
                cards.card_type as "card_type:CardTypeDb",
                cards.name,
                cards.memo
            FROM accounts
            JOIN cards ON accounts.id = cards.account_id
            WHERE accounts.id = $1
            "#,
            account_id
        )
        .fetch_one(tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to find card by id: {}", e);
            CardError::NotFound
        })?;
        Ok(Card {
            account: Account {
                id: row.account_id,
                owner_id: row.owner_id,
                created_at: row.created_at,
                updated_at: row.updated_at,
                deleted_at: row.deleted_at,
                account_type: crate::domain::enums::AccountType::Card,
            },
            card_number_last4: row.card_number_last4,
            encrypted_card_number: row.encrypted_card_number,
            issued_at: row.issued_at,
            expires_at: row.expires_at,
            billing_day: row.billing_day,
            brand: CardBrandDb::from(row.brand).into(),
            issuer: CardIssuerDb::from(row.issuer).into(),
            card_type: CardTypeDb::from(row.card_type).into(),
            name: row.name,
            memo: row.memo,
        })
    }
}

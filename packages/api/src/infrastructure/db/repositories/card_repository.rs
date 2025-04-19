use crate::{
    domain::{
        DomainError, Result,
        entities::Card,
        enums::{CardBrand, CardIssuer, CardType},
    },
    infrastructure::db::{ArcPgPool, schema},
};

use crate::domain::repositories::card_repository::CardRepository;
use async_trait::async_trait;
use uuid::Uuid;
pub struct CardRepositoryPostgres {
    pub pool: ArcPgPool,
}

impl CardRepositoryPostgres {
    pub fn new(pool: ArcPgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CardRepository for CardRepositoryPostgres {
    async fn save(&self, card: Card) -> Result<Card> {
        sqlx::query!(
            r#"INSERT INTO card (
                account_id, card_number_last4, encrypted_card_number,
                issued_at, expires_at, billing_day, credit_limit,
                brand, issuer, card_type
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)"#,
            card.account_id,
            card.card_number_last4,
            card.encrypted_card_number,
            card.issued_at,
            card.expires_at,
            card.billing_day,
            card.credit_limit,
            schema::CardBrand::from(card.brand) as schema::CardBrand,
            schema::CardIssuer::from(card.issuer) as schema::CardIssuer,
            schema::CardType::from(card.card_type) as schema::CardType,
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(card)
    }

    async fn find_by_account_id(&self, account_id: Uuid) -> Result<Card> {
        let row = sqlx::query_as_unchecked!(
            schema::Card,
            r#"
            SELECT 
                account_id, created_at, updated_at, deleted_at, 
                card_number_last4, encrypted_card_number, issued_at, 
                expires_at, billing_day, credit_limit, 
                brand as "brand: schema::CardBrand", 
                issuer as "issuer: schema::CardIssuer", 
                card_type as "card_type: schema::CardType"
            FROM card WHERE account_id = $1
            "#,
            account_id
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?
        .ok_or(DomainError::RepositoryError(format!(
            "Card with account_id {} not found",
            account_id
        )))?;
        Ok(Card::from(row))
    }

    async fn find_by_owner_id(&self, owner_id: Uuid) -> Result<Vec<Card>> {
        let rows = sqlx::query_as!(
            schema::Card,
            r#"
                SELECT 
                    c.account_id, 
                    c.created_at, 
                    c.updated_at, 
                    c.deleted_at, 
                    c.card_number_last4, 
                    c.encrypted_card_number, 
                    c.issued_at, 
                    c.expires_at, 
                    c.billing_day, 
                    c.credit_limit, 
                    c.brand as "brand: schema::CardBrand", 
                    c.issuer as "issuer: schema::CardIssuer", 
                    c.card_type as "card_type: schema::CardType"
                FROM card c
                JOIN account a ON c.account_id = a.id
                WHERE a.owner_id = $1
            "#,
            owner_id
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }

    async fn delete(&self, account_id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM card WHERE account_id = $1
            "#,
            account_id
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(())
    }
}

impl From<schema::Card> for Card {
    fn from(row: schema::Card) -> Self {
        Self {
            account_id: row.account_id,
            created_at: row.created_at,
            updated_at: row.updated_at,
            deleted_at: row.deleted_at,
            card_number_last4: row.card_number_last4,
            encrypted_card_number: row.encrypted_card_number,
            issued_at: row.issued_at,
            expires_at: row.expires_at,
            billing_day: row.billing_day,
            credit_limit: row.credit_limit,
            brand: row.brand.into(),
            issuer: row.issuer.into(),
            card_type: row.card_type.into(),
        }
    }
}

impl From<schema::CardIssuer> for CardIssuer {
    fn from(issuer: schema::CardIssuer) -> Self {
        match issuer {
            schema::CardIssuer::Samsung => CardIssuer::Samsung,
            schema::CardIssuer::BC => CardIssuer::BC,
            schema::CardIssuer::Woori => CardIssuer::Woori,
            schema::CardIssuer::Hana => CardIssuer::Hana,
            schema::CardIssuer::Shinhan => CardIssuer::Shinhan,
            schema::CardIssuer::Hyundai => CardIssuer::Hyundai,
            schema::CardIssuer::KB => CardIssuer::KB,
            schema::CardIssuer::Lotte => CardIssuer::Lotte,
            schema::CardIssuer::NH => CardIssuer::NH,
        }
    }
}

impl From<schema::CardBrand> for CardBrand {
    fn from(value: schema::CardBrand) -> Self {
        match value {
            schema::CardBrand::Visa => CardBrand::Visa,
            schema::CardBrand::Mastercard => CardBrand::Mastercard,
            schema::CardBrand::JCB => CardBrand::JCB,
            schema::CardBrand::UnionPay => CardBrand::UnionPay,
            schema::CardBrand::Amex => CardBrand::Amex,
            schema::CardBrand::Etc => CardBrand::Etc,
        }
    }
}

impl From<schema::CardType> for CardType {
    fn from(value: schema::CardType) -> Self {
        match value {
            schema::CardType::Credit => CardType::Credit,
            schema::CardType::Debit => CardType::Debit,
            schema::CardType::Prepaid => CardType::Prepaid,
        }
    }
}

impl From<CardBrand> for schema::CardBrand {
    fn from(value: CardBrand) -> Self {
        match value {
            CardBrand::Visa => schema::CardBrand::Visa,
            CardBrand::Mastercard => schema::CardBrand::Mastercard,
            CardBrand::Amex => schema::CardBrand::Amex,
            CardBrand::JCB => schema::CardBrand::JCB,
            CardBrand::UnionPay => schema::CardBrand::UnionPay,
            CardBrand::Etc => schema::CardBrand::Etc,
        }
    }
}

impl From<CardIssuer> for schema::CardIssuer {
    fn from(value: CardIssuer) -> Self {
        match value {
            CardIssuer::Samsung => schema::CardIssuer::Samsung,
            CardIssuer::Hyundai => schema::CardIssuer::Hyundai,
            CardIssuer::KB => schema::CardIssuer::KB,
            CardIssuer::Shinhan => schema::CardIssuer::Shinhan,
            CardIssuer::Lotte => schema::CardIssuer::Lotte,
            CardIssuer::Hana => schema::CardIssuer::Hana,
            CardIssuer::BC => schema::CardIssuer::BC,
            CardIssuer::NH => schema::CardIssuer::NH,
            CardIssuer::Woori => schema::CardIssuer::Woori,
        }
    }
}

impl From<CardType> for schema::CardType {
    fn from(value: CardType) -> Self {
        match value {
            CardType::Credit => schema::CardType::Credit,
            CardType::Debit => schema::CardType::Debit,
            CardType::Prepaid => schema::CardType::Prepaid,
        }
    }
}

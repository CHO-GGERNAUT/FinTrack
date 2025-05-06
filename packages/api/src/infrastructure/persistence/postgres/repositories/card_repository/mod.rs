pub mod model;
use sqlx::{Postgres, Transaction};

use crate::{
    application::errors::RepositoryError,
    domain::{
        card::{entities::Card, repository::CardRepository},
        shared::services::Hasher,
    },
    infrastructure::{
        config::Config,
        services::{AesGcmEncryptionService, SHA3HashService},
    },
};

pub use model::{CardBrandDb, CardIssuerDb, CardRow, CardStatusDb, CardTypeDb};
pub static ENTITY_TYPE: &str = "Card";

pub struct CardRepositoryPg<'a> {
    tx: &'a mut Transaction<'static, Postgres>,
}
impl<'a> CardRepositoryPg<'a> {
    pub fn new(tx: &'a mut Transaction<'static, Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait::async_trait]
impl<'a> CardRepository for CardRepositoryPg<'a> {
    async fn create(&mut self, card: Card) -> Result<Card, RepositoryError> {
        let tx = self.tx.as_mut();
        let v = Config::get().encryption_key.clone();
        let hash_service = SHA3HashService;
        let card_fingerprint = hash_service
            .hash(card.card_number().value())
            .map_err(|e| RepositoryError::invalid_data(e))?;

        let encryption_service = AesGcmEncryptionService::new(&v)
            .map_err(|e| RepositoryError::unexpected("Create Card", e))?;
        let card_row = CardRow::from_entity(&card, &encryption_service)
            .map_err(|e| RepositoryError::invalid_data(e))?;
        let result = sqlx::query_as!(
            CardRow,
            r#"
                INSERT INTO cards (
                    version,
                    id,
                    user_id,
                    card_number,
                    last_four_digits,
                    card_fingerprint,

                    card_type,
                    card_brand,
                    card_issuer,
                    status,

                    expiration_date,
                    issuance_date,

                    created_at,
                    updated_at,
                    deleted_at,
                    name
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
                RETURNING
                    version,
                    id,
                    user_id,
                    card_number,
                    last_four_digits,

                    card_type as "card_type:CardTypeDb",
                    card_brand as "card_brand:CardBrandDb",
                    card_issuer as "card_issuer:CardIssuerDb",
                    status as "card_status:CardStatusDb",
                    name,
                    expiration_date,
                    issuance_date,
                    created_at,
                    updated_at,
                    deleted_at
            "#,
            card_row.version as i64,
            card_row.id,
            card_row.user_id,
            card_row.card_number,
            card_row.last_four_digits,
            card_fingerprint,
            card_row.card_type as CardTypeDb,
            card_row.card_brand as CardBrandDb,
            card_row.card_issuer as CardIssuerDb,
            card_row.card_status as CardStatusDb,
            card_row.expiration_date,
            card_row.issuance_date,
            card_row.created_at,
            card_row.updated_at,
            card_row.deleted_at,
            card_row.name,
        )
        .fetch_one(tx)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {}", e);
            RepositoryError::Conflict {
                entity_type: ENTITY_TYPE,
                details: e.to_string(),
            }
        })?;
        let result = result
            .to_entity(&encryption_service)
            .map_err(|e| RepositoryError::invalid_data(e))?;
        Ok(result)
    }
}

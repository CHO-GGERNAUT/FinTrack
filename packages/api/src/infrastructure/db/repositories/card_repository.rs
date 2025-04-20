// // use crate::{
// //     domain::{
// //         entities::Card,
// //         errors::{DomainError, Result, card::CardError},
// //         repositories::CardRepository,
// //     },
// //     infrastructure::db::{
// //         ArcPgPool,
// //         schema::{CardBrandDb, CardIssuerDb, CardRow, CardTypeDb},
// //     },
// // };
// // use async_trait::async_trait;
// // use sqlx::{Executor, Postgres};
// // use uuid::Uuid;

// // pub async fn save_card<'e, E>(exec: E, card: &Card) -> Result<Card>
// // where
// //     E: sqlx::Executor<'e, Database = sqlx::postgres::Postgres>,
// // {
// //     let row = sqlx::query_as!(
// //         CardRow,
// //         r#"
// //                 INSERT
// //                     INTO card (
// //                         account_id,
// //                         card_number_last4,
// //                         encrypted_card_number,
// //                         issued_at,
// //                         expires_at,
// //                         billing_day,
// //                         credit_limit,
// //                         brand,
// //                         issuer,
// //                         card_type
// //                     )
// //                     VALUES
// //                         ($1,$2,$3,$4,$5,$6,$7,$8::card_brand,$9::card_issuer,$10::card_type)
// //                     RETURNING
// //                         account_id,
// //                         created_at,
// //                         updated_at,
// //                         deleted_at,
// //                         card_number_last4,
// //                         encrypted_card_number,
// //                         issued_at,
// //                         expires_at,
// //                         billing_day,
// //                         credit_limit,
// //                         brand as "brand:CardBrandDb",
// //                         issuer as "issuer:CardIssuerDb",
// //                         card_type as "card_type:CardTypeDb"
// //              "#,
// //         card.account_id,
// //         card.card_number_last4,
// //         card.encrypted_card_number,
// //         card.issued_at,
// //         card.expires_at,
// //         card.billing_day,
// //         card.credit_limit,
// //         CardBrandDb::from(card.brand) as CardBrandDb,
// //         CardIssuerDb::from(card.issuer) as CardIssuerDb,
// //         CardTypeDb::from(card.card_type) as CardTypeDb,
// //     )
// //     .fetch_one(exec)
// //     .await
// //     .map_err(|e| {
// //         tracing::error!("Failed to save card: {}", e);
// //         DomainError::CardError(CardError::DuplicateCard)
// //     })?;

// //     Ok(row.into())
// // }

// // pub async fn find_card_by_account_id<'e, E>(exec: E, account_id: Uuid) -> Result<Card>
// // where
// //     E: Executor<'e, Database = Postgres>,
// // {
// //     let row = sqlx::query_as!(
// //         CardRow,
// //         r#"
// //             SELECT
// //                 account_id,
// //                 created_at,
// //                 updated_at,
// //                 deleted_at,
// //                 card_number_last4,
// //                 encrypted_card_number,
// //                 issued_at,
// //                 expires_at,
// //                 billing_day,
// //                 credit_limit,
// //                 brand as "brand:CardBrandDb",
// //                 issuer as "issuer:CardIssuerDb",
// //                 card_type as "card_type:CardTypeDb"
// //             FROM
// //                 card
// //             WHERE
// //                 account_id = $1
// //         "#,
// //         account_id
// //     )
// //     .fetch_optional(exec)
// //     .await
// //     .map_err(|e| {
// //         tracing::error!("Failed to find card by account_id: {}", e);
// //         DomainError::CardError(CardError::CardNotFound)
// //     })?
// //     .ok_or_else(|| {
// //         tracing::error!("Card not found");
// //         DomainError::CardError(CardError::CardNotFound)
// //     })?;

// //     Ok(row.into())
// // }

// // pub async fn find_cards_by_owner_id<'e, E>(exec: E, owner_id: Uuid) -> Result<Vec<Card>>
// // where
// //     E: Executor<'e, Database = Postgres>,
// // {
// //     let rows = sqlx::query_as!(
// //         CardRow,
// //         r#"
// //             SELECT
// //                 c.account_id,
// //                 c.created_at,
// //                 c.updated_at,
// //                 c.deleted_at,
// //                 c.card_number_last4,
// //                 c.encrypted_card_number,
// //                 c.issued_at,
// //                 c.expires_at,
// //                 c.billing_day,
// //                 c.credit_limit,
// //                 c.brand as "brand:CardBrandDb",
// //                 c.issuer as "issuer:CardIssuerDb",
// //                 c.card_type as "card_type:CardTypeDb"
// //             FROM card c
// //             JOIN account a ON c.account_id = a.id
// //             WHERE a.owner_id = $1
// //         "#,
// //         owner_id
// //     )
// //     .fetch_all(exec)
// //     .await
// //     .map_err(|e| {
// //         tracing::error!("Failed to find cards by owner_id: {}", e);
// //         DomainError::CardError(CardError::CardNotFound)
// //     })?;

// //     Ok(rows.into_iter().map(Into::into).collect())
// // }

// // pub async fn delete_card<'e, E>(exec: E, account_id: Uuid) -> Result<()>
// // where
// //     E: Executor<'e, Database = Postgres>,
// // {
// //     sqlx::query!(r#"DELETE FROM card WHERE account_id = $1"#, account_id)
// //         .execute(exec)
// //         .await
// //         .map_err(|e| {
// //             tracing::error!("Failed to delete card: {}", e);
// //             DomainError::CardError(CardError::CardNotFound)
// //         })?;

// //     Ok(())
// // }

// // pub struct CardRepositoryPostgres {
// //     pub pool: ArcPgPool,
// // }

// // impl CardRepositoryPostgres {
// //     pub fn new(pool: ArcPgPool) -> Self {
// //         Self { pool }
// //     }
// // }

// // #[async_trait]
// // impl CardRepository for CardRepositoryPostgres {
// //     async fn save(&mut self, card: &Card) -> Result<Card> {
// //         save_card(&*self.pool, card).await
// //     }

// //     async fn find_by_account_id(&mut self, account_id: Uuid) -> Result<Card> {
// //         find_card_by_account_id(&*self.pool, account_id).await
// //     }

// //     async fn find_by_owner_id(&mut self, owner_id: Uuid) -> Result<Vec<Card>> {
// //         find_cards_by_owner_id(&*self.pool, owner_id).await
// //     }

// //     async fn delete(&mut self, account_id: Uuid) -> Result<()> {
// //         delete_card(&*self.pool, account_id).await
// //     }
// // }

// // // pub struct CardRepositoryPostgresTx<'tx, 'conn> {
// // //     tx: &'tx mut sqlx::Transaction<'conn, Postgres>,
// // // }

// // // impl<'tx, 'conn> CardRepositoryPostgresTx<'tx, 'conn> {
// // //     pub fn new(tx: &'tx mut sqlx::Transaction<'conn, Postgres>) -> Self {
// // //         Self { tx }
// // //     }
// // // }
// // pub struct CardRepositoryPostgresTx<'tx, 'conn> {
// //     tx: &'tx mut sqlx::Transaction<'conn, Postgres>,
// // }
// // impl<'tx, 'conn> CardRepositoryPostgresTx<'tx, 'conn> {
// //     pub fn new(tx: &'tx mut sqlx::Transaction<'conn, Postgres>) -> Self {
// //         Self { tx }
// //     }
// // }

// // #[async_trait]
// // impl<'tx, 'conn> CardRepository for CardRepositoryPostgresTx<'tx, 'conn> {
// //     async fn save(&mut self, card: &Card) -> Result<Card> {
// //         card_sql_helpers::save_card(&mut *self.tx, card).await
// //         save_card(&mut *self.tx, card).await
// //     }

// //     async fn find_by_account_id(&mut self, account_id: Uuid) -> Result<Card> {
// //         find_card_by_account_id(&mut *self.tx, account_id).await
// //     }

// //     async fn find_by_owner_id(&mut self, owner_id: Uuid) -> Result<Vec<Card>> {
// //         find_cards_by_owner_id(&mut *self.tx, owner_id).await
// //     }

// //     async fn delete(&mut self, account_id: Uuid) -> Result<()> {
// //         delete_card(&mut *self.tx, account_id).await
// //     }
// // }

// use std::sync::Arc;

// use async_trait::async_trait;
// use sqlx::{Postgres, Transaction};

// use crate::{
//     domain::{
//         entities::Card,
//         errors::{DomainError, Result, card::CardError},
//         repositories::CardRepository,
//     },
//     infrastructure::db::schema::{CardBrandDb, CardIssuerDb, CardRow, CardTypeDb},
// };

// pub struct CardRepositoryPostgres {
//     pub tx: Transaction<'static, Postgres>,
// }

// impl CardRepositoryPostgres {
//     pub fn new(tx: Transaction<'static, Postgres>) -> Self {
//         Self { tx }
//     }
// }

// #[async_trait]
// impl CardRepository for CardRepositoryPostgres {
//     async fn create(&mut self, card: &Card) -> Result<Card> {
//         let tx = self.tx.as_mut();

//         let row = sqlx::query_as!(
//             CardRow,
//             r#"
//                 INSERT
//                     INTO card (
//                         account_id,
//                         card_number_last4,
//                         encrypted_card_number,
//                         issued_at,
//                         expires_at,
//                         billing_day,
//                         credit_limit,
//                         brand,
//                         issuer,
//                         card_type
//                     )
//                     VALUES
//                         ($1,$2,$3,$4,$5,$6,$7,$8::card_brand,$9::card_issuer,$10::card_type)
//                     RETURNING
//                         account_id,
//                         created_at,
//                         updated_at,
//                         deleted_at,
//                         card_number_last4,
//                         encrypted_card_number,
//                         issued_at,
//                         expires_at,
//                         billing_day,
//                         credit_limit,
//                         brand as "brand:CardBrandDb",
//                         issuer as "issuer:CardIssuerDb",
//                         card_type as "card_type:CardTypeDb"
//              "#,
//             card.account_id,
//             card.card_number_last4,
//             card.encrypted_card_number,
//             card.issued_at,
//             card.expires_at,
//             card.billing_day,
//             card.credit_limit,
//             CardBrandDb::from(card.brand) as CardBrandDb,
//             CardIssuerDb::from(card.issuer) as CardIssuerDb,
//             CardTypeDb::from(card.card_type) as CardTypeDb,
//         )
//         .fetch_one(tx)
//         .await
//         .map_err(|e| {
//             tracing::error!("Failed to save card: {}", e);
//             DomainError::CardError(CardError::DuplicateCard)
//         })?;

//         Ok(row.into())
//     }
// }

use crate::{
    domain::{
        entities::Card,
        errors::{DomainError, Result, card::CardError},
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
    async fn create(&mut self, card: &Card) -> Result<Card> {
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
            DomainError::CardError(CardError::DuplicateCard)
        })?;

        Ok(row.into())
    }

    async fn delete(&mut self, account_id: Uuid) -> Result<()> {
        let tx = self.tx.as_mut();

        sqlx::query!(r#"DELETE FROM card WHERE account_id = $1"#, account_id)
            .execute(tx)
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete card: {}", e);
                DomainError::CardError(CardError::CardNotFound)
            })?;

        Ok(())
    }
}

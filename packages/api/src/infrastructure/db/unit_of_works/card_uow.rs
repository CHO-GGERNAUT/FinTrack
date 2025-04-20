use crate::{
    domain::{
        errors::{DomainError, Result, card::CardError},
        unit_of_works::{CardUnitOfWork, UnitOfWork},
    },
    infrastructure::db::{
        ArcPgPool,
        repositories::{AccountRepositoryPostgres, CardRepositoryPostgres},
    },
};
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

pub struct CardUnitOfWorkPostgres {
    tx: Transaction<'static, Postgres>,
}

impl CardUnitOfWorkPostgres {
    pub async fn new(pool: ArcPgPool) -> Result<Self> {
        let tx: Transaction<'static, Postgres> = pool.begin().await.map_err(|e| {
            tracing::error!("Failed to begin transaction: {}", e);
            DomainError::CardError(CardError::CardNotFound)
        })?;

        Ok(Self { tx })
    }
}

#[async_trait]
impl CardUnitOfWork for CardUnitOfWorkPostgres {
    type AccountRepo<'a> = AccountRepositoryPostgres<'a>;
    type CardRepo<'a> = CardRepositoryPostgres<'a>;

    fn card_repo(&mut self) -> CardRepositoryPostgres<'_> {
        CardRepositoryPostgres::new(&mut self.tx)
    }
    fn account_repo(&mut self) -> Self::AccountRepo<'_> {
        AccountRepositoryPostgres::new(&mut self.tx)
    }
}

#[async_trait]
impl UnitOfWork for CardUnitOfWorkPostgres {
    async fn commit(self) -> Result<()> {
        self.tx.commit().await.map_err(|e| {
            tracing::error!("Failed to commit transaction: {}", e);
            DomainError::CardError(CardError::CardNotFound)
        })?;
        Ok(())
    }

    async fn rollback(self) -> Result<()> {
        self.tx.rollback().await.map_err(|e| {
            tracing::error!("Failed to commit transaction: {}", e);
            DomainError::CardError(CardError::CardNotFound)
        })?;
        Ok(())
    }
}

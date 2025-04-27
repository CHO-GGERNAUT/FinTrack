use crate::{
    domain::{
        errors::DomainError,
        unit_of_works::{CardUnitOfWork, UnitOfWork},
    },
    infrastructure::{
        db::repositories::{AccountRepositoryPostgres, CardRepositoryPostgres},
        errors::InfraError,
    },
};
use async_trait::async_trait;
use sqlx::{PgPool, Postgres};

use super::base_uow::BaseUnitOfWork;

pub struct CardUnitOfWorkPostgres {
    base: BaseUnitOfWork<Postgres>,
}

impl CardUnitOfWorkPostgres {
    pub async fn new(pool: PgPool) -> Result<Self, InfraError> {
        let tx = pool.begin().await.map_err(|e| {
            tracing::error!("Failed to begin transaction: {}", e);
            InfraError::DatabaseError(e.to_string())
        })?;

        Ok(Self {
            base: BaseUnitOfWork::new(tx),
        })
    }
}

#[async_trait]
impl CardUnitOfWork for CardUnitOfWorkPostgres {
    type AccountRepo<'a> = AccountRepositoryPostgres<'a>;
    type CardRepo<'a> = CardRepositoryPostgres<'a>;

    fn card_repo(&mut self) -> CardRepositoryPostgres<'_> {
        let tx = self.base.tx();
        CardRepositoryPostgres::new(tx)
    }
    fn account_repo(&mut self) -> Self::AccountRepo<'_> {
        let tx = self.base.tx();
        AccountRepositoryPostgres::new(tx)
    }
}

#[async_trait]
impl UnitOfWork for CardUnitOfWorkPostgres {
    async fn commit(self) -> Result<(), DomainError> {
        self.base.commit().await
    }

    async fn rollback(self) -> Result<(), DomainError> {
        self.base.rollback().await
    }
}

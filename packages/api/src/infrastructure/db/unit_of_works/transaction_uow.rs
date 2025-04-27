use crate::{
    domain::{
        errors::DomainError,
        unit_of_works::{TransactionUnitOfWork, UnitOfWork},
    },
    infrastructure::{
        db::repositories::{AccountRepositoryPostgres, TransactionRepositoryPostgres},
        errors::InfraError,
    },
};
use async_trait::async_trait;
use sqlx::{PgPool, Postgres};

use super::base_uow::BaseUnitOfWork;

pub struct TransactionUnitOfWorkPostgres {
    base: BaseUnitOfWork<Postgres>,
}

impl TransactionUnitOfWorkPostgres {
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
impl TransactionUnitOfWork for TransactionUnitOfWorkPostgres {
    type AccountRepo<'a> = AccountRepositoryPostgres<'a>;
    type TransactionRepo<'a> = TransactionRepositoryPostgres<'a>;

    fn account_repo(&mut self) -> Self::AccountRepo<'_> {
        let tx = self.base.tx();
        AccountRepositoryPostgres::new(tx)
    }

    fn transaction_repo(&mut self) -> Self::TransactionRepo<'_> {
        let tx = self.base.tx();
        TransactionRepositoryPostgres::new(tx)
    }
}

#[async_trait]
impl UnitOfWork for TransactionUnitOfWorkPostgres {
    async fn commit(self) -> Result<(), DomainError> {
        self.base.commit().await
    }

    async fn rollback(self) -> Result<(), DomainError> {
        self.base.rollback().await
    }
}

use crate::{
    domain::{
        errors::DomainError,
        unit_of_works::{CardTransactionUnitOfWork, UnitOfWork},
    },
    infrastructure::{
        db::repositories::{
            AccountRepositoryPostgres, CardTransactionRepositoryPostgres,
            MerchantRepositoryPostgres,
        },
        errors::InfraError,
    },
};
use async_trait::async_trait;
use sqlx::{PgPool, Postgres};

use super::base_uow::BaseUnitOfWork;

pub struct CardTransactionUnitOfWorkPostgres {
    base: BaseUnitOfWork<Postgres>,
}

impl CardTransactionUnitOfWorkPostgres {
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
impl CardTransactionUnitOfWork for CardTransactionUnitOfWorkPostgres {
    type AccountRepo<'a> = AccountRepositoryPostgres<'a>;
    type CardTransactionRepo<'a> = CardTransactionRepositoryPostgres<'a>;
    type MerchantRepo<'a> = MerchantRepositoryPostgres<'a>;

    fn account_repo(&mut self) -> Self::AccountRepo<'_> {
        let tx = self.base.tx();
        AccountRepositoryPostgres::new(tx)
    }

    fn transaction_repo(&mut self) -> Self::CardTransactionRepo<'_> {
        let tx = self.base.tx();
        CardTransactionRepositoryPostgres::new(tx)
    }

    fn merchant_repo(&mut self) -> Self::MerchantRepo<'_> {
        let tx = self.base.tx();
        MerchantRepositoryPostgres::new(tx)
    }
}

#[async_trait]
impl UnitOfWork for CardTransactionUnitOfWorkPostgres {
    async fn commit(self) -> Result<(), DomainError> {
        self.base.commit().await
    }

    async fn rollback(self) -> Result<(), DomainError> {
        self.base.rollback().await
    }
}

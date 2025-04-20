use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

use crate::domain::{
    errors::{DomainError, Result},
    unit_of_works::UnitOfWork,
};

pub struct UnitOfWorkPostgres {
    tx: Option<Transaction<'static, Postgres>>,
}
impl UnitOfWorkPostgres {
    pub fn new(tx: Transaction<'static, Postgres>) -> Self {
        Self { tx: Some(tx) }
    }
}

#[async_trait]
impl UnitOfWork for UnitOfWorkPostgres {
    async fn commit(mut self) -> Result<()> {
        if let Some(tx) = self.tx.take() {
            tx.commit()
                .await
                .map_err(|e| DomainError::CommonError(e.to_string()))
        } else {
            Ok(())
        }
    }

    async fn rollback(mut self) -> Result<()> {
        if let Some(tx) = self.tx.take() {
            tx.rollback()
                .await
                .map_err(|e| DomainError::CommonError(e.to_string()))
        } else {
            Ok(())
        }
    }
}

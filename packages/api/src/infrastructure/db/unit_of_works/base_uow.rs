use sqlx::{Database, Transaction};

use crate::domain::errors::{DomainError, UowError};

pub struct BaseUnitOfWork<D: Database> {
    tx: Option<Transaction<'static, D>>,
}

impl<D: Database> BaseUnitOfWork<D> {
    pub fn new(tx: Transaction<'static, D>) -> Self {
        Self { tx: Some(tx) }
    }

    pub fn tx(&mut self) -> &mut Transaction<'static, D> {
        self.tx.as_mut().expect("Transaction already finished")
    }

    pub async fn commit(mut self) -> Result<(), DomainError> {
        if let Some(tx) = self.tx.take() {
            tx.commit()
                .await
                .map_err(|e| UowError::CommitError(e.to_string()).into())
        } else {
            Ok(())
        }
    }

    pub async fn rollback(mut self) -> Result<(), DomainError> {
        if let Some(tx) = self.tx.take() {
            tx.rollback()
                .await
                .map_err(|e| UowError::RollbackError(e.to_string()).into())
        } else {
            Ok(())
        }
    }
}

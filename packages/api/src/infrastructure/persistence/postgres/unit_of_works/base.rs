use sqlx::{Postgres, Transaction};

use crate::application::errors::RepositoryError;

pub struct BaseUnitOfWorkPg {
    tx: Option<Transaction<'static, Postgres>>,
}

impl BaseUnitOfWorkPg {
    pub fn new(tx: Transaction<'static, Postgres>) -> Self {
        Self { tx: Some(tx) }
    }

    pub fn tx(&mut self) -> &mut Transaction<'static, Postgres> {
        self.tx.as_mut().expect("Transaction already finished")
    }

    pub async fn commit(mut self) -> Result<(), RepositoryError> {
        if let Some(tx) = self.tx.take() {
            tx.commit().await.map_err(|e| RepositoryError::db(e))?;
            Ok(())
        } else {
            Err(RepositoryError::unexpected(
                "Commit",
                sqlx::Error::PoolClosed,
            ))
        }
    }
}

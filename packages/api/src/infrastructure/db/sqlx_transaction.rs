// use async_trait::async_trait;
// use sqlx::{Pool, Postgres, Transaction as SqlxTransaction};
// use std::{any::Any, error::Error};

// use crate::domain::transactions::{Transaction, TransactionManager};

// pub struct SqlxTransactionManager {
//     pool: Pool<Postgres>,
// }

// impl SqlxTransactionManager {
//     pub fn new(pool: Pool<Postgres>) -> Self {
//         Self { pool }
//     }
// }

// pub struct SqlxTransactionImpl {
//     tx: SqlxTransaction<'static, Postgres>,
// }

// #[async_trait]
// impl Transaction for SqlxTransactionImpl {
//     async fn commit(self: Box<Self>) -> Result<(), Box<dyn Error>> {
//         self.tx.commit().await.map_err(|e| e.into())
//     }

//     async fn rollback(self: Box<Self>) -> Result<(), Box<dyn Error>> {
//         self.tx.rollback().await.map_err(|e| e.into())
//     }

//     fn as_any_mut(&mut self) -> &mut dyn Any {
//         self
//     }
// }

// #[async_trait]
// impl TransactionManager for SqlxTransactionManager {
//     async fn begin_transaction(&self) -> Result<Box<dyn Transaction>, Box<dyn Error>> {
//         let tx = self.pool.begin().await?;
//         Ok(Box::new(SqlxTransactionImpl { tx }))
//     }
// }

use async_trait::async_trait;
use sqlx::{Pool, Postgres, Transaction as SqlxTransaction};
use std::error::Error;

use crate::domain::transactions::{Transaction, TransactionManager};

pub struct SqlxTransactionImpl<'a> {
    pub tx: SqlxTransaction<'a, Postgres>,
}

#[async_trait]
impl<'a> Transaction for SqlxTransactionImpl<'a> {
    async fn commit(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        self.tx.commit().await.map_err(Into::into)
    }

    async fn rollback(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        self.tx.rollback().await.map_err(Into::into)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct SqlxTransactionManager {
    pub pool: Pool<Postgres>,
}

impl SqlxTransactionManager {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TransactionManager for SqlxTransactionManager {
    async fn begin_transaction<'a>(&'a self) -> Result<Box<dyn Transaction + 'a>, Box<dyn Error>> {
        let tx = self.pool.begin().await?;
        Ok(Box::new(SqlxTransactionImpl { tx }))
    }
}

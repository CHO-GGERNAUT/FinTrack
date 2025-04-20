// use async_trait::async_trait;
// use std::{any::Any, error::Error};

// #[async_trait]
// pub trait Transaction: Any + Send + Sync {
//     async fn commit(self: Box<Self>) -> Result<(), Box<dyn Error>>;
//     async fn rollback(self: Box<Self>) -> Result<(), Box<dyn Error>>;

//     // Any로 다운캐스팅을 위한 메서드
//     fn as_any(&self) -> &dyn Any;
//     fn as_any_mut(&mut self) -> &mut dyn Any;
// }

// #[async_trait]
// pub trait TransactionManager: Send + Sync {
//     async fn begin_transaction(&self) -> Result<Box<dyn Transaction>, Box<dyn Error>>;
// }

// use async_trait::async_trait;
// use std::error::Error;

// #[async_trait]
// pub trait Transaction<'a> {
//     async fn commit(self: Box<Self>) -> Result<(), Box<dyn Error>>;
//     async fn rollback(self: Box<Self>) -> Result<(), Box<dyn Error>>;
//     fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
// }

// #[async_trait]
// pub trait TransactionManager {
//     async fn begin_transaction<'a>(
//         &'a self,
//     ) -> Result<Box<dyn Transaction<'a> + 'a>, Box<dyn Error>>;
// }

use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Transaction {
    async fn commit(self: Box<Self>) -> Result<(), Box<dyn Error>>;
    async fn rollback(self: Box<Self>) -> Result<(), Box<dyn Error>>;
}

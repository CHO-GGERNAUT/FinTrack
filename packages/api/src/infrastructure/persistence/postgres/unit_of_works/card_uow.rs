use sqlx::PgPool;

use crate::{
    application::{
        errors::RepositoryError,
        interfaces::unit_of_works::{CardUnitOfWork, UnitOfWork},
    },
    infrastructure::persistence::postgres::repositories::CardRepositoryPg,
};

use super::BaseUnitOfWorkPg;

pub struct CardUnitOfWorkPg {
    base: BaseUnitOfWorkPg,
}

impl CardUnitOfWorkPg {
    pub async fn new(pool: PgPool) -> Result<Self, RepositoryError> {
        let tx = pool.begin().await.map_err(|e| RepositoryError::db(e))?;
        Ok(Self {
            base: BaseUnitOfWorkPg::new(tx),
        })
    }
}

impl CardUnitOfWork for CardUnitOfWorkPg {
    type CardRepo<'a> = CardRepositoryPg<'a>;

    fn card_repository(&mut self) -> Self::CardRepo<'_> {
        let tx = self.base.tx();
        CardRepositoryPg::new(tx)
    }
}

#[async_trait::async_trait]
impl UnitOfWork for CardUnitOfWorkPg {
    async fn commit(self) -> Result<(), RepositoryError> {
        self.base.commit().await
    }
}

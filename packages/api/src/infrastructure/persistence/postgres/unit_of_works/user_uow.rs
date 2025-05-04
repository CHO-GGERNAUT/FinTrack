use sqlx::PgPool;

use crate::{
    application::{
        errors::RepositoryError,
        interfaces::unit_of_works::{UnitOfWork, UserUnitOfWork},
    },
    infrastructure::persistence::postgres::repositories::{
        PasswordCredentialRepositoryPg, UserRepositoryPg,
    },
};

use super::BaseUnitOfWorkPg;

pub struct UserUnitOfWorkPg {
    base: BaseUnitOfWorkPg,
}

impl UserUnitOfWorkPg {
    pub async fn new(pool: PgPool) -> Result<Self, sqlx::Error> {
        let tx = pool.begin().await?;
        Ok(Self {
            base: BaseUnitOfWorkPg::new(tx),
        })
    }
}

impl UserUnitOfWork for UserUnitOfWorkPg {
    type UserRepo<'a> = UserRepositoryPg<'a>;
    type PasswordCredentialRepo<'a> = PasswordCredentialRepositoryPg<'a>;

    fn user_repository(&mut self) -> Self::UserRepo<'_> {
        let tx = self.base.tx();
        UserRepositoryPg::new(tx)
    }

    fn password_credential_repository(&mut self) -> Self::PasswordCredentialRepo<'_> {
        let tx = self.base.tx();
        PasswordCredentialRepositoryPg::new(tx)
    }
}

#[async_trait::async_trait]
impl UnitOfWork for UserUnitOfWorkPg {
    async fn commit(self) -> Result<(), RepositoryError> {
        self.base.commit().await
    }
}

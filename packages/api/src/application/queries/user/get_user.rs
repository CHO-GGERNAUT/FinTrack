use uuid::Uuid;

use crate::{
    application::{errors::ApplicationError, interfaces::unit_of_works::UserUnitOfWork},
    domain::user::{entities::User, repository::UserRepository},
};

pub struct GetUserQuery {
    pub user_id: Uuid,
}

pub struct GetUserResult {
    pub user: User,
}

pub struct GetUserHandler<U: UserUnitOfWork> {
    uow: U,
}

impl<U: UserUnitOfWork> GetUserHandler<U> {
    pub fn new(uow: U) -> Self {
        Self { uow }
    }

    pub async fn execute(mut self, query: GetUserQuery) -> Result<GetUserResult, ApplicationError> {
        let user_id = query.user_id.into();
        let user = self.uow.user_repository().find_by_id(user_id).await?;
        self.uow.commit().await?;
        Ok(GetUserResult { user })
    }
}

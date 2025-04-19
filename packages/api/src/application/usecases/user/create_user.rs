use crate::domain::{entities::user::User, repositories::user_repository::UserRepository};

#[derive(Clone)]
pub struct CreateUserUsecase<R: UserRepository> {
    pub repo: R,
}

impl<R: UserRepository> CreateUserUsecase<R> {
    pub async fn execute(&self, user: User) -> anyhow::Result<()> {
        self.repo.save(user).await
    }
}

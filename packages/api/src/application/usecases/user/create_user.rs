use uuid::Uuid;

use crate::{
    application::{
        dto::{CreateUserInput, CreateUserOutput},
        error::{ApplicationError, Result},
    },
    domain::{entities::User, repositories::user_repository::UserRepository},
};

#[derive(Clone)]
pub struct CreateUserUsecase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> CreateUserUsecase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, input: CreateUserInput) -> Result<CreateUserOutput> {
        let user: User = input.into();

        self.repo.save(user).await.map_err(|e| {
            ApplicationError::RepositoryError(format!("Failed to save user: {}", e))
        })?;

        // 4. 결과를 DTO로 변환하여 반환
        Ok(CreateUserOutput {
            user_id: Uuid::new_v4(),
            created_at: String::default(),
        })
    }
}

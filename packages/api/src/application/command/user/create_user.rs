use crate::{
    application::{
        dto::{CreateUserInput, CreateUserOutput},
        errors::ApplicationError,
    },
    domain::{entities::User, repositories::UserRepository},
    utils::crypto::hash_password,
};

#[derive(Clone)]
pub struct CreateUserUsecase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> CreateUserUsecase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(
        mut self,
        input: CreateUserInput,
    ) -> Result<CreateUserOutput, ApplicationError> {
        let hashed = hash_password(&input.password)
            .map_err(|e| ApplicationError::HashError(format!("Failed to hash password: {}", e)))?;
        let user = User {
            password: hashed,
            ..input.into()
        };

        self.repo.create(&user).await?;

        Ok(CreateUserOutput::from(user))
    }
}

use crate::{
    application::{
        dto::{CreateUserInput, CreateUserOutput},
        errors::ApplicationError,
        services::AuthService,
    },
    domain::{entities::User, repositories::UserRepository},
};

#[derive(Clone)]
pub struct CreateUserUsecase<R: UserRepository, A: AuthService> {
    repo: R,
    auth: A,
}

impl<R: UserRepository, A: AuthService> CreateUserUsecase<R, A> {
    pub fn new(repo: R, auth: A) -> Self {
        Self { repo, auth }
    }

    pub async fn execute(
        mut self,
        input: CreateUserInput,
    ) -> Result<CreateUserOutput, ApplicationError> {
        let hashed = self.auth.hash_password(&input.password)?;

        let user = User {
            password: hashed,
            ..input.into()
        };

        self.repo.create(&user).await?;

        Ok(CreateUserOutput::from(user))
    }
}

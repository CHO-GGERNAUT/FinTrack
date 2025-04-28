use crate::{
    application::{
        dto::{IssueTokenInput, IssueTokenOutput},
        errors::ApplicationError,
        services::AuthService,
    },
    domain::repositories::UserRepository,
};

#[derive(Clone)]
pub struct IssueTokenUsecase<R: UserRepository, A: AuthService> {
    repo: R,
    auth: A,
}

impl<R: UserRepository, A: AuthService> IssueTokenUsecase<R, A> {
    pub fn new(repo: R, auth: A) -> Self {
        Self { repo, auth }
    }

    pub async fn execute(
        mut self,
        input: IssueTokenInput,
    ) -> Result<IssueTokenOutput, ApplicationError> {
        let user = self.repo.find_by_email(&input.email).await?;

        self.auth.verify_password(&input.password, &user.password)?;

        let token = self.auth.issue_access_token(user.id)?;

        Ok(IssueTokenOutput {
            token,
            user_id: user.id,
        })
    }
}

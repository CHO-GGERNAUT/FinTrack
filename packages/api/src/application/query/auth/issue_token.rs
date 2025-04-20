use crate::{
    application::{
        dto::{Claims, IssueTokenInput, IssueTokenOutput},
        error::ApplicationError,
        error::Result,
        services::JwtService,
    },
    domain::repositories::UserRepository,
};

#[derive(Clone)]
pub struct IssueTokenUsecase<R: UserRepository, J: JwtService> {
    repo: R,
    jwt: J,
}

impl<R: UserRepository, J: JwtService> IssueTokenUsecase<R, J> {
    pub fn new(repo: R, jwt: J) -> Self {
        Self { repo, jwt }
    }

    pub async fn execute(mut self, input: IssueTokenInput) -> Result<IssueTokenOutput> {
        let user = self.repo.find_by_email(&input.email).await?;

        if !self.verify_password(&user.password, &input.password) {
            return Err(ApplicationError::ValidationError(
                "Invalid email or password".to_string(),
            ));
        }

        let claims = Claims {
            sub: user.email.clone(),
            user_id: user.id.to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
        };

        let token = self
            .jwt
            .generate(&claims)
            .map_err(|e| ApplicationError::JwtError(format!("Failed to generate token: {}", e)))?;

        Ok(IssueTokenOutput {
            token,
            user_id: user.id,
        })
    }

    fn verify_password(&self, hashed: &str, plain: &str) -> bool {
        match bcrypt::verify(plain, hashed) {
            Ok(is_valid) => is_valid,
            Err(_) => false,
        }
    }
}

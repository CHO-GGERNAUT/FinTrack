use crate::domain::{entities::user::User, repositories::user_repository::UserRepository};
use anyhow::{Result, anyhow};

pub struct LoginUserUsecase<R: UserRepository> {
    pub repo: R,
}

impl<R: UserRepository> LoginUserUsecase<R> {
    pub async fn execute(&self, email: &str, password: &str) -> Result<User> {
        let user = self
            .repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;

        if !verify_password(&user.password, password) {
            return Err(anyhow!("Invalid credentials"));
        }

        Ok(user)
    }
}

fn verify_password(hashed: &str, plain: &str) -> bool {
    bcrypt::verify(plain, hashed).unwrap_or(false)
}

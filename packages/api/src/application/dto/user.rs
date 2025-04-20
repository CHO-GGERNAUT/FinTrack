use chrono::Utc;
use uuid::Uuid;

use crate::domain::entities::User;

#[derive(Debug)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct CreateUserOutput {
    pub user_id: Uuid,
    pub created_at: String,
}

impl From<CreateUserInput> for User {
    fn from(input: CreateUserInput) -> Self {
        User {
            id: Uuid::new_v4(),
            name: input.name,
            email: input.email,
            password: input.password,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl From<User> for CreateUserOutput {
    fn from(user: User) -> Self {
        Self {
            user_id: user.id,
            created_at: user.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug)]
pub struct IssueTokenInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct IssueTokenOutput {
    pub token: String,
    pub user_id: Uuid,
}

impl From<User> for IssueTokenOutput {
    fn from(user: User) -> Self {
        Self {
            token: user.password, // In a real application, this would be a JWT or similar
            user_id: user.id,
        }
    }
}

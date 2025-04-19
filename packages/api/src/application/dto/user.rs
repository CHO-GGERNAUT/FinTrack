use uuid::Uuid;

use crate::domain;

#[derive(Debug)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct CreateUserOutput {
    pub user_id: Uuid,
    pub created_at: String, // ISO 8601
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

impl From<CreateUserInput> for domain::entities::User {
    fn from(input: CreateUserInput) -> Self {
        domain::entities::User {
            id: Uuid::new_v4(),
            name: input.name,
            email: input.email,
            password: input.password,
        }
    }
}

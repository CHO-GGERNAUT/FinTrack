use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    application::commands::user::{RegisterUserPasswordCommand, RegisterUserPasswordResult},
    domain::user::entities::User,
};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub phone_number: String,
}

impl From<CreateUserRequest> for RegisterUserPasswordCommand {
    fn from(req: CreateUserRequest) -> Self {
        Self {
            email: req.email,
            password: req.password,
            phone_number: req.phone_number,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub id: Uuid,
}

impl From<RegisterUserPasswordResult> for CreateUserResponse {
    fn from(output: RegisterUserPasswordResult) -> Self {
        Self { id: output.user_id }
    }
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub phone_number: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id().as_deref(),
            email: user.email().to_string(),
            phone_number: user.phone_number().as_str().to_string(),
        }
    }
}

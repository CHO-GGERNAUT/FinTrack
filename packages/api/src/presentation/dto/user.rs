use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::dto::{CreateUserInput, CreateUserOutput, IssueTokenInput};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub id: Uuid,
    pub created_at: String,
}

impl From<CreateUserRequest> for CreateUserInput {
    fn from(req: CreateUserRequest) -> Self {
        Self {
            name: req.name,
            email: req.email,
            password: req.password,
        }
    }
}

impl From<CreateUserOutput> for CreateUserResponse {
    fn from(output: CreateUserOutput) -> Self {
        Self {
            id: output.user_id,
            created_at: output.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl From<LoginRequest> for IssueTokenInput {
    fn from(req: LoginRequest) -> Self {
        Self {
            email: req.email,
            password: req.password,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
}

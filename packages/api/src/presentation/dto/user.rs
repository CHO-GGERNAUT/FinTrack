use crate::application::dto::{
    CreateUserInput, CreateUserOutput, IssueTokenInput, IssueTokenOutput,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub user_id: Uuid,
    pub created_at: String, // ISO 8601
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub user_id: Uuid,
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
            user_id: output.user_id,
            created_at: output.created_at,
        }
    }
}

impl From<LoginRequest> for IssueTokenInput {
    fn from(req: LoginRequest) -> Self {
        Self {
            email: req.email,
            password: req.password,
        }
    }
}

impl From<IssueTokenOutput> for LoginResponse {
    fn from(output: IssueTokenOutput) -> Self {
        Self {
            user_id: output.user_id,
        }
    }
}

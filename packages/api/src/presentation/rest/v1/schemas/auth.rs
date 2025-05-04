use serde::{Deserialize, Serialize};

use crate::application::commands::auth::{PasswordAuthenticateCommand, PasswordAuthenticateResult};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl From<LoginRequest> for PasswordAuthenticateCommand {
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
    pub refresh_token: String,
}

impl From<PasswordAuthenticateResult> for LoginResponse {
    fn from(output: PasswordAuthenticateResult) -> Self {
        Self {
            access_token: output.access_token,
            refresh_token: output.refresh_token,
        }
    }
}

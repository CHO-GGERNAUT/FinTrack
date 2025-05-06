use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::collections::HashMap;
use thiserror::Error;

use crate::application::errors::ApplicationError;

// RFC 7807 Problem Details
#[derive(Serialize)]
struct ProblemDetail {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_uri: Option<String>,
    title: String,
    status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_errors: Option<HashMap<String, Vec<String>>>,
}

#[derive(Error, Debug)]
pub enum RestApiError {
    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Authentication failed")]
    Authentication,

    #[error(transparent)]
    Application(#[from] ApplicationError),
}

impl IntoResponse for RestApiError {
    fn into_response(self) -> Response {
        if matches!(
            self,
            RestApiError::Application(ApplicationError::InternalError { .. })
        ) {
            tracing::error!(error = ?self, "API Internal Error Occurred");
        }

        let (status, title, detail, validation_errors) = match self {
            RestApiError::BadRequest(message) => (
                StatusCode::BAD_REQUEST,
                "Bad Request".to_string(),
                Some(message),
                None,
            ),
            RestApiError::Authentication => (
                StatusCode::UNAUTHORIZED,
                "UNAUTHORIZED".to_string(),
                None,
                None,
            ),
            RestApiError::Application(app_err) => match app_err {
                ApplicationError::Validation(err) => (
                    StatusCode::UNPROCESSABLE_ENTITY, // 422 사용
                    "Validation Failed".to_string(),
                    Some(err.to_string()),
                    None,
                ),
                ApplicationError::NotFound(err) => (
                    StatusCode::NOT_FOUND,
                    "Resource Not Found".to_string(),
                    Some(err.to_string()),
                    None,
                ),
                ApplicationError::Conflict(err) => (
                    StatusCode::CONFLICT,
                    "Conflict".to_string(),
                    Some(err.to_string()),
                    None,
                ),
                ApplicationError::Authentication(err) => (
                    StatusCode::UNAUTHORIZED,
                    "Authentication Failed".to_string(),
                    Some(err.to_string()),
                    None,
                ),
                ApplicationError::InternalError(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                    Some(err.to_string()),
                    None,
                ),
            },
        };

        let problem = ProblemDetail {
            type_uri: None,
            title,
            status: status.as_u16(),
            detail,
            instance: None,
            validation_errors,
        };

        (status, Json(problem)).into_response()
    }
}

pub type RestApiResult<T> = Result<T, RestApiError>;

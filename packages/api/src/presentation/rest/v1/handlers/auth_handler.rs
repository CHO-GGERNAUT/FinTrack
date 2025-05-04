use std::sync::Arc;

use axum::{
    Extension, Json,
    http::{StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Response},
};
use sqlx::PgPool;

use crate::{
    application::commands::auth::PasswordAuthenticateHandler,
    infrastructure::{
        config::Config, persistence::postgres::unit_of_works::UserUnitOfWorkPg,
        services::TokenServiceImpl,
    },
    presentation::rest::v1::schemas::auth::LoginRequest,
};

pub async fn login_handler(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Response, (StatusCode, String)> {
    let token_service = Arc::new(TokenServiceImpl::new(&Config::get().jwt_secret));
    // let auth = AuthServiceImpl::new(&Config::get().jwt_secret);
    let uow = UserUnitOfWorkPg::new(pool).await.map_err(|e| {
        tracing::error!("Failed to create user unit of work: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    })?;
    let handler = PasswordAuthenticateHandler::new(uow, token_service);

    let output = handler
        .execute(req.into())
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    let cookie = cookie::CookieBuilder::new("access_token", output.access_token)
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::Lax)
        .build();

    Ok(([(SET_COOKIE, cookie.to_string())], Json("Login success")).into_response())
}

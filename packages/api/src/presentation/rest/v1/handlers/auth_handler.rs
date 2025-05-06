use std::sync::Arc;

use axum::{
    Extension, Json,
    http::header::SET_COOKIE,
    response::{IntoResponse, Response},
};
use sqlx::PgPool;

use crate::{
    application::{commands::auth::PasswordAuthenticateHandler, errors::ApplicationError},
    infrastructure::{
        config::Config,
        persistence::postgres::unit_of_works::UserUnitOfWorkPg,
        services::{BcryptHashService, JwtService},
    },
    presentation::rest::{error::RestApiResult, v1::schemas::auth::LoginRequest},
};

pub async fn login_handler(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<LoginRequest>,
) -> RestApiResult<Response> {
    let token_service = Arc::new(JwtService::new(&Config::get().jwt_secret));
    let hash_service = BcryptHashService;
    let uow = UserUnitOfWorkPg::new(pool)
        .await
        .map_err(|e| ApplicationError::from(e))?;
    let handler = PasswordAuthenticateHandler::new(uow, token_service, hash_service);

    let res = handler.execute(req.into()).await?;

    let cookie = cookie::CookieBuilder::new("access_token", res.access_token)
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::Lax)
        .build();

    Ok(([(SET_COOKIE, cookie.to_string())], Json("Login success")).into_response())
}

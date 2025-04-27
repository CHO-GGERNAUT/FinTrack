use axum::{
    Extension, Json,
    http::{StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Response},
};
use sqlx::PgPool;

use crate::{
    application::{
        command::user::CreateUserUsecase,
        dto::{CreateUserInput, IssueTokenInput},
        query::auth::IssueTokenUsecase,
    },
    infrastructure::{
        config::Config, db::repositories::UserRepositoryPostgresPool, services::jwt::JwtServiceImpl,
    },
    presentation::schemas::user::{CreateUserRequest, CreateUserResponse, LoginRequest},
};

pub async fn register_handler(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, (StatusCode, String)> {
    let usecase = CreateUserUsecase::new(UserRepositoryPostgresPool::new(pool));
    let res = usecase
        .execute(CreateUserInput::from(req))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(res.into()))
}

pub async fn login_handler(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Response, (StatusCode, String)> {
    let jwt = JwtServiceImpl::new(&Config::get().jwt_secret);
    let usecase = IssueTokenUsecase::new(UserRepositoryPostgresPool::new(pool), jwt);

    let output = usecase
        .execute(IssueTokenInput::from(req))
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    let cookie = cookie::CookieBuilder::new("auth_token", output.token)
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::Lax)
        .build();

    Ok(([(SET_COOKIE, cookie.to_string())], Json("Login success")).into_response())
}

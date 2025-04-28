use axum::{
    Extension, Json,
    http::{StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Response},
};
use sqlx::PgPool;

use crate::{
    application::{dto::IssueTokenInput, query::auth::IssueTokenUsecase},
    infrastructure::{
        config::Config, db::repositories::UserRepositoryPostgresPool,
        services::auth::AuthServiceImpl,
    },
    presentation::schemas::user::LoginRequest,
};

pub async fn login_handler(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Response, (StatusCode, String)> {
    let auth = AuthServiceImpl::new(&Config::get().jwt_secret);
    let usecase = IssueTokenUsecase::new(UserRepositoryPostgresPool::new(pool), auth);

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

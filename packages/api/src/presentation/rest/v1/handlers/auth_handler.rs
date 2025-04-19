use axum::{
    Extension, Json,
    http::{StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Response},
};

use crate::{
    application::{
        dto::CreateUserInput,
        usecases::user::{CreateUserUsecase, IssueTokenUsecase},
    },
    infrastructure::{
        config::Config,
        db::{ArcPgPool, repositories::UserRepositoryPostgres},
        services::jwt::JwtServiceImpl,
    },
    presentation::dto::user::{CreateUserRequest, CreateUserResponse, LoginRequest},
};

pub async fn register_handler(
    Extension(pool): Extension<ArcPgPool>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, (StatusCode, String)> {
    let hashed = bcrypt::hash(req.password, 10)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let usecase = CreateUserUsecase::new(UserRepositoryPostgres::new(pool));
    let res = usecase
        .execute(CreateUserInput {
            name: req.name,
            email: req.email,
            password: hashed,
        })
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(res.into()))
}

pub async fn login_handler(
    Extension(pool): Extension<ArcPgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Response, (StatusCode, String)> {
    let jwt = JwtServiceImpl::new(&Config::get().jwt_secret);
    let usecase = IssueTokenUsecase::new(UserRepositoryPostgres::new(pool), jwt);

    let output = usecase
        .execute(req.into())
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    let cookie = cookie::CookieBuilder::new("auth_token", output.token)
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::Lax)
        .build();

    Ok(([(SET_COOKIE, cookie.to_string())], Json("Login success")).into_response())
}

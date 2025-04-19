use axum::{
    Extension, Json,
    http::{StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Response},
};
use models::dto::user::{CreateUserRequest, LoginRequest};

use crate::{
    application::{
        services::jwt_service::JwtService,
        usecases::user::{CreateUserUsecase, LoginUserUsecase},
    },
    domain::entities::user::User,
    infrastructure::{
        config::Config,
        db::{ArcPgPool, user_repository::UserRepositoryPostgres},
    },
};

pub async fn register_handler(
    Extension(pool): Extension<ArcPgPool>,
    Json(req): Json<CreateUserRequest>,
) -> Result<(), (StatusCode, String)> {
    let hashed = bcrypt::hash(req.password, 10)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let user = User {
        id: uuid::Uuid::new_v4(),
        name: req.name,
        email: req.email,
        password: hashed,
    };
    let usecase = CreateUserUsecase {
        repo: UserRepositoryPostgres { pool },
    };
    usecase
        .execute(user)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn login_handler(
    Extension(pool): Extension<ArcPgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Response, (StatusCode, String)> {
    let usecase = LoginUserUsecase {
        repo: UserRepositoryPostgres { pool },
    };

    let user = usecase
        .execute(&req.email, &req.password)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    let config = Config::get();
    let jwt_service = JwtService::new(config.jwt_secret.clone());
    let token = jwt_service.generate(&user.id.to_string()).unwrap();

    let cookie = cookie::CookieBuilder::new("auth_token", token)
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::Lax)
        .build();

    Ok(([(SET_COOKIE, cookie.to_string())], Json("Login success")).into_response())
}

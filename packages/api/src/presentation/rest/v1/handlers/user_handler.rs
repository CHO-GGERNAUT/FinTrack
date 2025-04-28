use axum::{Extension, Json, http::StatusCode};
use sqlx::PgPool;

use crate::{
    application::{command::users::CreateUserUsecase, dto::CreateUserInput},
    infrastructure::{
        config::Config, db::repositories::UserRepositoryPostgresPool,
        services::auth::AuthServiceImpl,
    },
    presentation::schemas::user::{CreateUserRequest, CreateUserResponse},
};

pub async fn create_user_handler(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, (StatusCode, String)> {
    let auth = AuthServiceImpl::new(&Config::get().jwt_secret);
    let usecase = CreateUserUsecase::new(UserRepositoryPostgresPool::new(pool), auth);
    let res = usecase
        .execute(CreateUserInput::from(req))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(res.into()))
}

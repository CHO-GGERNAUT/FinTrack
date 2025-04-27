use axum::{Extension, Json, http::StatusCode};

use crate::{
    application::{command::user::CreateUserUsecase, dto::CreateUserInput},
    infrastructure::db::{ArcPgPool, repositories::UserRepositoryPostgresPool},
    presentation::schemas::user::{CreateUserRequest, CreateUserResponse},
};

pub async fn create_user_handler(
    Extension(pool): Extension<ArcPgPool>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, (StatusCode, String)> {
    let usecase = CreateUserUsecase::new(UserRepositoryPostgresPool::new(pool));
    let res = usecase
        .execute(CreateUserInput::from(req))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(res.into()))
}

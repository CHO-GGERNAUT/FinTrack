use axum::{Extension, Json};
use sqlx::PgPool;

use crate::{
    application::{
        commands::user::RegisterUserPasswordHandler,
        errors::ApplicationError,
        interfaces::services::token_service::Claims,
        queries::user::{GetUserHandler, GetUserQuery},
    },
    infrastructure::{
        persistence::postgres::unit_of_works::UserUnitOfWorkPg, services::BcryptHashService,
    },
    presentation::rest::{
        error::{RestApiError, RestApiResult},
        v1::schemas::user::{CreateUserRequest, CreateUserResponse, UserResponse},
    },
};

pub async fn create_user_handler(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<CreateUserRequest>,
) -> RestApiResult<Json<CreateUserResponse>> {
    let uow = UserUnitOfWorkPg::new(pool)
        .await
        .map_err(|e| ApplicationError::from(e))?;

    let hash_service = BcryptHashService;
    let handler = RegisterUserPasswordHandler::new(uow, hash_service);
    let res = handler.execute(req.into()).await?;
    Ok(Json(res.into()))
}

pub async fn get_user_handler(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Option<Claims>>,
) -> RestApiResult<Json<UserResponse>> {
    tracing::debug!("Claims: {:?}", claims);
    let user_id_str = claims
        .as_ref()
        .ok_or(RestApiError::Authentication)?
        .sub
        .clone(); // Clone sub to avoid borrowing issues if parse fails early

    let user_id = user_id_str
        .parse::<uuid::Uuid>()
        .map_err(|_| RestApiError::BadRequest("Invalid user ID format in token".to_string()))?;

    let uow = UserUnitOfWorkPg::new(pool)
        .await
        .map_err(|e| ApplicationError::from(e))?;
    let handler = GetUserHandler::new(uow);
    let res = handler.execute(GetUserQuery { user_id }).await?;
    Ok(Json(res.user.into()))
}

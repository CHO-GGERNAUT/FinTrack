use axum::{Extension, Json, http::StatusCode};
use uuid::Uuid;

use crate::{
    application::{dto::Claims, usecases::card::CreateCardUseCase},
    infrastructure::db::{ArcPgPool, repositories::CardRepositoryPostgres},
    presentation::dto::card::{CreateCardRequest, CreateCardResponse},
};

pub async fn create_card(
    Extension(_claims): Extension<Option<Claims>>,
    Extension(pool): Extension<ArcPgPool>,
    Json(req): Json<CreateCardRequest>,
) -> Result<Json<CreateCardResponse>, (StatusCode, String)> {
    let usecase = CreateCardUseCase {
        repo: CardRepositoryPostgres { pool },
    };
    usecase
        .execute(req.try_into().map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Failed to convert request: {:?}", e),
            )
        })?)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    // let result = usecase.execute().await?;

    Ok(Json(CreateCardResponse {
        account_id: Uuid::new_v4(),
        created_at: String::default(),
    }))
}

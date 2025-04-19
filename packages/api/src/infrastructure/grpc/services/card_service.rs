use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::{
    application::usecases::card::{CreateCardUseCase, DeleteCardUseCase, GetCardsUseCase},
    domain::repositories::CardRepository,
    infrastructure::grpc::proto::card::{
        CardListResponse, CardResponse, CreateCardRequest, DeleteCardRequest, DeleteCardResponse,
        GetCardsRequest, card_service_server::CardService,
    },
};

pub struct CardServiceImpl<T: CardRepository> {
    card_repository: T,
}

impl<T: CardRepository> CardServiceImpl<T> {
    pub fn new(card_repository: T) -> Self {
        Self { card_repository }
    }
}

#[tonic::async_trait]
impl<T: CardRepository + Send + Sync + 'static> CardService for CardServiceImpl<T> {
    async fn create_card(
        &self,
        request: Request<CreateCardRequest>,
    ) -> Result<Response<CardResponse>, Status> {
        let req = request.into_inner();
        let owner_id = Uuid::parse_str(&req.owner_id)
            .map_err(|_| Status::invalid_argument("Invalid owner_id format"))?;

        // Convert gRPC request to DTO
        // Implementation will depend on your protobuf definitions

        let use_case = CreateCardUseCase::new(self.card_repository.clone());
        let result = use_case
            .execute(owner_id, dto)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Convert result to gRPC response
        // Implementation will depend on your protobuf definitions

        Ok(Response::new(response))
    }

    async fn get_cards(
        &self,
        request: Request<GetCardsRequest>,
    ) -> Result<Response<CardListResponse>, Status> {
        let req = request.into_inner();
        let owner_id = Uuid::parse_str(&req.owner_id)
            .map_err(|_| Status::invalid_argument("Invalid owner_id format"))?;

        let use_case = GetCardsUseCase::new(self.card_repository.clone());
        let result = use_case
            .execute(owner_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Convert result to gRPC response
        // Implementation will depend on your protobuf definitions

        Ok(Response::new(response))
    }

    async fn delete_card(
        &self,
        request: Request<DeleteCardRequest>,
    ) -> Result<Response<DeleteCardResponse>, Status> {
        let req = request.into_inner();
        let account_id = Uuid::parse_str(&req.account_id)
            .map_err(|_| Status::invalid_argument("Invalid account_id format"))?;

        let use_case = DeleteCardUseCase::new(self.card_repository.clone());
        use_case
            .execute(account_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(DeleteCardResponse {}))
    }
}

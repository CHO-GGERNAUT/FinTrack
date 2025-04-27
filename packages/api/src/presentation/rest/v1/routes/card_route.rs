use crate::presentation::rest::v1::handlers::{
    card_handler::{create_card_handler, delete_card_handler},
    card_transaction_handler::create_card_transaction_handler,
};
use axum::{Router, routing::post};

pub fn routes() -> Router {
    Router::new()
        .route("/", post(create_card_handler).delete(delete_card_handler))
        .route(
            "/:card_id/transactions",
            post(create_card_transaction_handler),
        )
}

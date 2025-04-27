use crate::presentation::rest::v1::handlers::transaction_handler::create_transaction_handler;
use axum::{Router, routing::post};

pub fn routes() -> Router {
    Router::new().route("/", post(create_transaction_handler))
}

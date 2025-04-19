use crate::presentation::rest::v1::handlers::auth_handler::{login_handler, register_handler};
use axum::{Router, routing::post};

pub fn routes() -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
}

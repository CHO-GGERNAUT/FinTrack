use axum::Router;

mod handlers {
    pub mod auth_handler;
    pub mod card_handler;
    pub mod transaction_handler;
    pub mod user_handler;
}
pub mod routes {
    pub mod auth_route;
    pub mod card_route;
    pub mod transaction_route;
    pub mod user_route;
}

pub fn routes() -> Router {
    Router::new()
        .nest("/auth", routes::auth_route::routes())
        .nest("/user", routes::user_route::routes())
        .nest("/card", routes::card_route::routes())
        .nest("/transaction", routes::transaction_route::routes())
}

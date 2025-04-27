use axum::Router;

mod handlers {
    pub mod auth_handler;
    pub mod card_handler;
    pub mod card_transaction_handler;
    pub mod user_handler;
}
pub mod routes {
    pub mod auth_route;
    pub mod card_route;
    pub mod user_route;
}

pub fn routes() -> Router {
    Router::new()
        .nest("/auths", routes::auth_route::routes())
        .nest("/users", routes::user_route::routes())
        .nest("/cards", routes::card_route::routes())
}

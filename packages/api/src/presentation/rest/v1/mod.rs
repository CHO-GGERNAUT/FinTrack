use axum::Router;

mod handlers {
    pub mod auth_handler;
    pub mod card_handler;
}
pub mod routes {
    pub mod auth_route;
    pub mod card_route;
}

pub fn routes() -> Router {
    Router::new()
        .nest("/auth", routes::auth_route::routes())
        .nest("/card", routes::card_route::routes())
}

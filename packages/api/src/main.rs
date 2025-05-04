use std::sync::Arc;

use application::interfaces::services::TokenService;
use axum::middleware;
use infrastructure::{
    config::Config, persistence::postgres::connection::create_pool, services::TokenServiceImpl,
};
use presentation::rest::middlewares::auth_middleware;
use tokio::net::TcpListener;

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

#[tokio::main]
async fn main() {
    let config = Config::get();
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();

    #[cfg(feature = "rest")]
    {
        use axum::Extension;
        use presentation::rest::v1;

        let token_service: Arc<dyn TokenService> =
            Arc::new(TokenServiceImpl::new(&config.jwt_secret));
        let pool = create_pool().expect("Failed to create database pool");
        let listener = TcpListener::bind(format!("0.0.0.0:{}", config.server_port))
            .await
            .unwrap();

        let app = axum::Router::new()
            .nest("/v1", v1::routes())
            .layer(middleware::from_fn(auth_middleware))
            .layer(Extension(pool))
            .layer(Extension(token_service));

        println!("Server started on {:?}", listener.local_addr().unwrap());
        match axum::serve(listener, app).await {
            Ok(_) => {
                tracing::info!("Server Terminated");
            }
            Err(err) => {
                tracing::error!("Server error: {:?}", err);
            }
        }
    }
}

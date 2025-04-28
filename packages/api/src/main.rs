pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use std::sync::Arc;

use application::services::AuthService;
use axum::middleware;
use infrastructure::{
    config::Config, db::create_pool::create_pool, middleware::auth_middleware::auth_middleware,
    services::auth::AuthServiceImpl,
};

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

    let pool = create_pool();

    #[cfg(feature = "rest")]
    {
        use axum::Extension;
        use presentation::rest::routes;
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.server_port))
            .await
            .unwrap();
        let auth: Arc<dyn AuthService> = Arc::new(AuthServiceImpl::new(&config.jwt_secret));

        println!("Server started on {:?}", listener.local_addr().unwrap());
        let app: axum::Router = routes()
            .layer(middleware::from_fn(auth_middleware))
            .layer(Extension(auth))
            .layer(Extension(pool));

        match axum::serve(listener, app).await {
            Ok(_) => {
                tracing::info!("Server Terminated");
            }
            Err(err) => {
                tracing::error!("Server error: {:?}", err);
            }
        }
    }
    #[cfg(feature = "grpc")]
    {
        unimplemented!("gRPC server not implemented yet");
    }
}

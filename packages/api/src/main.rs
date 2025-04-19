use core::infrastructure::db::create_pool;
use std::sync::Arc;

use config::Config;

mod config;
mod rest;
mod core {
    pub mod application;
    pub mod domain;
    pub mod infrastructure;
}

mod utils;

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
    let pool: Arc<sqlx::Pool<sqlx::Postgres>> = Arc::new(pool);

    #[cfg(feature = "rest")]
    {
        use axum::Extension;
        use rest::router;
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.server_port))
            .await
            .unwrap();

        println!("Server started on {:?}", listener.local_addr().unwrap());
        let app = router().layer(Extension(pool));

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
    // Initialize API router
}

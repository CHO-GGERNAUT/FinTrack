use std::sync::Arc;

mod application {
    pub mod services;
    pub mod usecases;
}
mod domain {
    pub mod entities;
    pub mod enums;
    pub mod repositories;
}
mod infrastructure {
    pub mod config;
    pub mod db;
    pub mod middleware;
}
mod presentation {
    pub mod grpc;
    pub mod rest;
}
mod utils;

use infrastructure::{config::Config, db::create_pool};

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
        use presentation::rest::router;
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

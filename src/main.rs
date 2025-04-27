mod di;

use axum::{Extension, serve};
use std::{net::SocketAddr, sync::Arc};

use crate::di::postgres::initialize_pool;
use crate::di::router::{AppUserRepository, router};
use application::service::UserService;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize the database connection pool
    let pool = initialize_pool().await?;

    // Initialize the repository and service for dependency injection
    let repo = Arc::new(AppUserRepository::new(pool));
    let service = Arc::new(UserService::new(repo));

    // Create the router with the service
    let app = router().layer(Extension(service));

    // Serve the application
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Listening on {}", addr);
    serve::serve(listener, app.into_make_service()).await?;

    Ok(())
}

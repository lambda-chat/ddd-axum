mod di;

use axum::{Extension, serve};
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};

use crate::di::router::router;
use application::service::UserService;
use infrastructure::persistence::SqlxUserRepository;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize the database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://admin:admin@localhost:5432/postgres-db")
        .await?;

    // Initialize the repository and service for dependency injection
    let repo = Arc::new(SqlxUserRepository::new(pool));
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

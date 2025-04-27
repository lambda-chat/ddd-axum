mod di;

use axum::{Extension, serve};
use std::{net::SocketAddr, sync::Arc};

use crate::di::handlers::AppUserRepository;
use crate::di::router::router;
use application::service::UserService;

#[cfg(not(feature = "testing"))]
use sqlx::postgres::PgPoolOptions;
#[cfg(feature = "testing")]
use sqlx::{Pool, Postgres};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    #[cfg(not(feature = "testing"))]
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://admin:admin@localhost:5432/postgres-db")
        .await?;
    #[cfg(feature = "testing")]
    let pool = Pool::<Postgres>::connect_lazy("postgres://mock").unwrap();

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

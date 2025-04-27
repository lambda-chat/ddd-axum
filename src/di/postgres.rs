use sqlx::{Postgres, postgres::PgPoolOptions};

pub async fn initialize_pool() -> Result<sqlx::Pool<Postgres>, sqlx::Error> {
    #[cfg(not(feature = "testing"))]
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://admin:admin@localhost:5432/postgres-db")
        .await;
    #[cfg(feature = "testing")]
    let pool = sqlx::Pool::<Postgres>::connect_lazy("postgres://mock")?;
    pool
}

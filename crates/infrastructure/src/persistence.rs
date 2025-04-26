use async_trait::async_trait;
use domain::{error::DomainError, repository::UserRepository, user::User};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone)]
pub struct SqlxUserRepository {
    pool: Pool<Postgres>,
}

impl SqlxUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<User, DomainError> {
        let rec = sqlx::query!("SELECT id, name, created_at FROM users WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| DomainError::NotFound)?;

        Ok(User {
            id: rec.id,
            name: rec.name,
            created_at: rec.created_at,
        })
    }

    async fn save(&self, user: &User) -> Result<(), DomainError> {
        sqlx::query!(
            "INSERT INTO users (id, name, created_at) VALUES ($1, $2, $3)",
            user.id,
            user.name,
            user.created_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::PersistenceError(e.to_string()))?;

        Ok(())
    }
}

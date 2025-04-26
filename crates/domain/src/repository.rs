use crate::{error::DomainError, user::User};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<User, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;
}

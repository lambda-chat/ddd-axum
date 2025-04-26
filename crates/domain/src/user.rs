use crate::error::DomainError;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(name: String) -> Result<Self, DomainError> {
        if name.len() < 1 {
            return Err(DomainError::NameTooShort);
        }
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            created_at: Utc::now(),
        })
    }
}

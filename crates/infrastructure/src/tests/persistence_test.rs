use async_trait::async_trait;
use domain::{error::DomainError, repository::UserRepository, user::User};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct TestUserRepository {
    users: Arc<Mutex<HashMap<Uuid, User>>>,
}

impl TestUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn seed(&self, users: Vec<User>) -> &Self {
        let mut users_map = self.users.lock().unwrap();
        for user in users {
            users_map.insert(user.id, user);
        }
        self
    }
}

#[async_trait]
impl UserRepository for TestUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<User, DomainError> {
        let users = self.users.lock().unwrap();
        users.get(&id).cloned().ok_or(DomainError::NotFound)
    }

    async fn save(&self, user: &User) -> Result<(), DomainError> {
        let mut users = self.users.lock().unwrap();
        users.insert(user.id, user.clone());
        Ok(())
    }
}

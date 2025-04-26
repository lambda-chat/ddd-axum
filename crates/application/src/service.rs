use super::dto::{CreateUserDto, UserDto};
use domain::{error::DomainError, repository::UserRepository, user::User};
use std::sync::Arc;
use uuid::Uuid;

pub struct UserService<R: UserRepository> {
    repo: Arc<R>,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, dto: CreateUserDto) -> Result<UserDto, DomainError> {
        let user = User::new(dto.name)?;
        self.repo.save(&user).await?;
        Ok(UserDto::from(user))
    }

    pub async fn get_user(&self, id: Uuid) -> Result<UserDto, DomainError> {
        let user = self.repo.find_by_id(id).await?;
        Ok(UserDto::from(user))
    }
}

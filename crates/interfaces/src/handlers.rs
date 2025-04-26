use application::dto::{CreateUserDto, UserDto};
use application::service::UserService;
use axum::{Extension, Json, extract::Path, http::StatusCode};
use domain::error::DomainError;
use domain::repository::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_user<R>(
    Json(payload): Json<CreateUserDto>,
    Extension(service): Extension<Arc<UserService<R>>>,
) -> Result<Json<UserDto>, StatusCode>
where
    R: UserRepository + 'static,
{
    service
        .create_user(payload)
        .await
        .map(Json)
        .map_err(|err| match err {
            DomainError::NameTooShort => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

pub async fn get_user<R>(
    Path(id): Path<Uuid>,
    Extension(service): Extension<Arc<UserService<R>>>,
) -> Result<Json<UserDto>, StatusCode>
where
    R: UserRepository + 'static,
{
    service
        .get_user(id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

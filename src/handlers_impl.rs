use application::dto::{CreateUserDto, UserDto};
use application::service::UserService;
use axum::Router;
use axum::routing::{get, post};
use axum::{Extension, Json, extract::Path, http::StatusCode};
use infrastructure::persistence::SqlxUserRepository;
use std::sync::Arc;
use uuid::Uuid;

use interfaces::handlers::{create_user, get_user};

pub async fn create_user_impl(
    extension: Extension<Arc<UserService<SqlxUserRepository>>>,
    json: Json<CreateUserDto>,
) -> Result<Json<UserDto>, StatusCode> {
    return create_user::<SqlxUserRepository>(json, extension).await;
}

pub async fn get_user_impl(
    extension: Extension<Arc<UserService<SqlxUserRepository>>>,
    path: Path<Uuid>,
) -> Result<Json<UserDto>, StatusCode> {
    return get_user::<SqlxUserRepository>(path, extension).await;
}

pub fn router() -> Router {
    Router::new()
        .route("/users", post(create_user_impl))
        .route("/users/{:id}", get(get_user_impl))
}

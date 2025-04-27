use application::dto::{CreateUserDto, UserDto};
use application::service::UserService;
use axum::Router;
use axum::routing::{get, post};
use axum::{Extension, Json, extract::Path, http::StatusCode};
use infrastructure::persistence::SqlxUserRepository;
use std::sync::Arc;
use uuid::Uuid;

#[rustfmt::skip::macros(pub_async_def_di)]
macro_rules! pub_async_def_di {
    (
        $func_name:ident,                  // wrapper function name
        $($seg:ident)::+,                  // function name
        $repo:ty,                          // dependency
        ( $($arg:ident : $ty:ty),* $(,)? ) // function arguments
    ) => {
        pub async fn $func_name(
            Extension(service): Extension<Arc<UserService<SqlxUserRepository>>>,
            $($arg: $ty),*
        ) -> Result<Json<UserDto>, StatusCode> {
            $($seg)::*::<SqlxUserRepository>($($arg),*, Extension(Arc::clone(&service))).await
        }
    };
}

pub_async_def_di!(
    create_user, interfaces::handlers::create_user,
    SqlxUserRepository,
    (json: Json<CreateUserDto>,)
);

pub_async_def_di!(
    get_user, interfaces::handlers::get_user,
    SqlxUserRepository,
    (path: Path<Uuid>,)
);

pub fn router() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/{id}", get(get_user))
}

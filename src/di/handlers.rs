use application::dto::{CreateUserDto, UserDto};
use application::service::UserService;
use axum::{Extension, Json, extract::Path, http::StatusCode};
use infrastructure::persistence::SqlxUserRepository;
use std::sync::Arc;
use uuid::Uuid;

#[rustfmt::skip::macros(inject)]
macro_rules! inject {
    (
        $func_name:ident,                  // wrapper function name
        $($seg:ident)::+,                  // function name
        $repo:ty,                          // dependency
        ( $($arg:ident : $ty:ty),* $(,)? ) // function arguments
    ) => {
        pub async fn $func_name(
            extension: Extension<Arc<UserService<$repo>>>,
            $($arg: $ty),*
        ) -> Result<Json<UserDto>, StatusCode> {
            $($seg)::*::<$repo>($($arg),*, extension).await
        }
    };
}

// Example of macro expansion
//
// pub async def create_user(
//     extension: Extension<Arc<UserService<SqlxUserRepository>>>,
//     json: Json<CreateUserDto>,
// ) -> Result<Json<UserDto>, StatusCode> {
//     interfaces::handlers::create_user::<SqlxUserRepository>(json, extension)).await
// }

inject!(create_user, interfaces::handlers::create_user, SqlxUserRepository, (json: Json<CreateUserDto>,));
inject!(get_user, interfaces::handlers::get_user, SqlxUserRepository, (path: Path<Uuid>,));

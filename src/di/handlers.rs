use application::dto::{CreateUserDto, UserDto};
use application::service::UserService;
use axum::{Extension, Json, extract::Path, http::StatusCode};
use std::sync::Arc;
use uuid::Uuid;

#[cfg(not(feature = "testing"))]
use infrastructure::persistence::SqlxUserRepository as AppUserRepository;
#[cfg(feature = "testing")]
use infrastructure::tests::persistence_test::TestUserRepository as AppUserRepository;

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
// ```rs
// inject!(create_user, interfaces::handlers::create_user, AppUserRepository, (json: Json<CreateUserDto>,));
// ```
//
// ```rs
// pub async def create_user(
//     extension: Extension<Arc<UserService<SqlxUserRepository>>>,
//     json: Json<CreateUserDto>,
// ) -> Result<Json<UserDto>, StatusCode> {
//     interfaces::handlers::create_user::<SqlxUserRepository>(json, extension)).await
// }
// ```

inject!(create_user, interfaces::handlers::create_user, AppUserRepository, (json: Json<CreateUserDto>,));
inject!(get_user, interfaces::handlers::get_user, AppUserRepository, (path: Path<Uuid>,));

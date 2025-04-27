use axum::Router;

use axum::routing::{get, post};

#[cfg(not(feature = "testing"))]
pub use infrastructure::persistence::SqlxUserRepository as AppUserRepository;
#[cfg(feature = "testing")]
pub use infrastructure::tests::persistence_test::TestUserRepository as AppUserRepository;

pub fn router() -> Router {
    Router::new()
        .route(
            "/users",
            post(interfaces::handlers::create_user::<AppUserRepository>),
        )
        .route(
            "/users/{id}",
            get(interfaces::handlers::get_user::<AppUserRepository>),
        )
}

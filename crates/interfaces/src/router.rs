use super::handlers::{create_user, get_user};
use axum::Router;
use axum::routing::{get, post};

pub fn router<R>() -> Router
where
    R: domain::repository::UserRepository + Send + Sync + 'static,
{
    Router::new()
        // .route("/users", post(create_user::<R>)) // FIXME: cannot compile
        .route("/users/{:id}", get(get_user::<R>))
}

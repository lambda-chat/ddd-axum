use axum::Router;

use crate::di::handlers::{create_user, get_user};
use axum::routing::{get, post};

pub fn router() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/{id}", get(get_user))
}

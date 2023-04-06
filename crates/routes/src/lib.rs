mod error;
mod post;
use std::sync::Arc;

use axum::extract::FromRef;
use axum::Router;

type PostService = Arc<dyn service::post::PostService + Send + Sync>;

#[derive(FromRef, Clone)]
pub struct RouterState {
    pub post_service: PostService,
}

pub fn router(state: RouterState) -> Router {
    Router::new()
        .route("/post/:id", axum::routing::get(post::get))
        .route("/post/:id", axum::routing::delete(post::delete))
        .route("/post", axum::routing::get(post::get_all))
        .route("/post", axum::routing::post(post::post))
        .with_state(state)
}

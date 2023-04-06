use crate::error::ApiError;
use crate::PostService;

use axum::debug_handler;
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::WithRejection;
use model::post::Post;

#[debug_handler]
pub async fn get_all(State(service): State<PostService>) -> Result<Json<Vec<Post>>, ApiError> {
    service.get_all().await.map(Json).map_err(Into::into)
}

pub async fn get(
    State(service): State<PostService>,
    WithRejection(Path(id), _): WithRejection<Path<String>, ApiError>,
) -> Result<Json<Post>, ApiError> {
    service.get(id).await.map(Json).map_err(Into::into)
}

pub async fn post(
    State(service): State<PostService>,
    WithRejection(Json(post), _): WithRejection<Json<Post>, ApiError>,
) -> Result<(), ApiError> {
    service.new(post).await.map_err(Into::into)
}

pub async fn delete(
    State(service): State<PostService>,
    WithRejection(Path(id), _): WithRejection<Path<String>, ApiError>,
) -> Result<(), ApiError> {
    service.delete(id).await.map_err(Into::into)
}

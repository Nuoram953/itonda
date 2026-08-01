use axum::{
    body::Body,
    extract::{Path, State},
    response::Response,
};
use itonda_database::media::find_asset_by_id;
use itonda_domain::storage::path::AppPaths;
use mime_guess::from_path;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use tracing::instrument;
use uuid::Uuid;

use crate::{api::error::ApiError, state::AppState};

#[utoipa::path(
    get,
    path = "/assets/{id}",
    params(
        ("id" = String, Path, description = "Asset id")
    ),
    responses(
        (status = 200, description = "Asset file"),
        (status = 404, description = "Asset not found")
    )
)]
#[instrument(skip(state))]
pub async fn get_asset(
    State(state): State<AppState>,
    Path(asset_id): Path<String>,
) -> Result<Response, ApiError> {
    let asset = find_asset_by_id(&state.db, asset_id)
        .await?
        .ok_or(ApiError::AssetNotFound)?;

    let media_id = Uuid::parse_str(&asset.media_id).unwrap();

    let path = AppPaths::new().media_dir(media_id).join(asset.path);

    let file = File::open(path.clone())
        .await
        .map_err(|_| ApiError::AssetNotFound)?;

    let stream = ReaderStream::new(file);

    let content_type = from_path(&path).first_or_octet_stream();

    Ok(Response::builder()
        .header("content-type", content_type.as_ref())
        .body(Body::from_stream(stream))
        .unwrap())
}

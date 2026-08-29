use axum::{
    body::Body,
    extract::{Path, State},
    response::Response,
};
use itonda_database::media::find_asset_by_id;
use itonda_domain::{assets::downloader::AssetDownloader, storage::path::AppPaths};
use mime_guess::from_path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
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

    let mut file = File::open(path.clone())
        .await
        .map_err(|_| ApiError::AssetNotFound)?;

    let mut header_buf = [0u8; 16];
    let n = file.read(&mut header_buf).await.unwrap_or(0);
    let detected_type = AssetDownloader::detect_image_extension(&header_buf[..n]).map(|ext| {
        match ext {
            "jpg" => "image/jpeg",
            "png" => "image/png",
            "webp" => "image/webp",
            "gif" => "image/gif",
            "avif" => "image/avif",
            "svg" => "image/svg+xml",
            _ => "application/octet-stream",
        }
    });

    let fallback_mime = from_path(&path).first_or_octet_stream();
    let content_type = detected_type.unwrap_or_else(|| fallback_mime.as_ref());

    let _ = file.seek(std::io::SeekFrom::Start(0)).await;

    let stream = ReaderStream::new(file);

    Ok(Response::builder()
        .header("content-type", content_type)
        .body(Body::from_stream(stream))
        .unwrap())
}

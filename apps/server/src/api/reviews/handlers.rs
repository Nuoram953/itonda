use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use itonda_domain::reviews::{
    models::{GameReview, GameReviewOverview, GameThought},
    service as ReviewService,
};
use tracing::instrument;

use crate::{
    api::{
        error::ApiError,
        extractor::AppJson,
        reviews::schemas::{CreateThoughtPayload, UpdateThoughtPayload, UpsertReviewPayload},
    },
    state::AppState,
};

#[utoipa::path(
    get,
    path = "/media/{media_id}/review",
    params(
        ("media_id" = String, Path, description = "Media ID"),
    ),
    responses(
        (
            status = 200,
            body = GameReviewOverview,
            description = "Game review and thoughts overview"
        )
    )
)]
#[instrument(skip(state))]
pub async fn get_review_overview(
    State(state): State<AppState>,
    Path(media_id): Path<String>,
) -> Result<Json<GameReviewOverview>, ApiError> {
    let overview = ReviewService::get_review_overview(&state.db, &media_id).await?;
    Ok(Json(overview))
}

#[utoipa::path(
    put,
    path = "/media/{media_id}/review",
    params(
        ("media_id" = String, Path, description = "Media ID"),
    ),
    request_body = UpsertReviewPayload,
    responses(
        (
            status = 200,
            body = GameReview,
            description = "Upserted game review"
        )
    )
)]
#[instrument(skip(state, payload))]
pub async fn upsert_review(
    State(state): State<AppState>,
    Path(media_id): Path<String>,
    AppJson(payload): AppJson<UpsertReviewPayload>,
) -> Result<Json<GameReview>, ApiError> {
    let review =
        ReviewService::upsert_review(&state.db, &media_id, payload.verdict, payload.summary)
            .await?;
    Ok(Json(review))
}

#[utoipa::path(
    delete,
    path = "/media/{media_id}/review",
    params(
        ("media_id" = String, Path, description = "Media ID"),
    ),
    responses(
        (
            status = 204,
            description = "Review successfully deleted"
        )
    )
)]
#[instrument(skip(state))]
pub async fn delete_review(
    State(state): State<AppState>,
    Path(media_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    ReviewService::delete_review(&state.db, &media_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/media/{media_id}/thoughts",
    params(
        ("media_id" = String, Path, description = "Media ID"),
    ),
    responses(
        (
            status = 200,
            body = Vec<GameThought>,
            description = "List of thoughts for the game"
        )
    )
)]
#[instrument(skip(state))]
pub async fn get_thoughts(
    State(state): State<AppState>,
    Path(media_id): Path<String>,
) -> Result<Json<Vec<GameThought>>, ApiError> {
    let thoughts = ReviewService::get_thoughts(&state.db, &media_id).await?;
    Ok(Json(thoughts))
}

#[utoipa::path(
    post,
    path = "/media/{media_id}/thoughts",
    params(
        ("media_id" = String, Path, description = "Media ID"),
    ),
    request_body = CreateThoughtPayload,
    responses(
        (
            status = 201,
            body = GameThought,
            description = "Created thought"
        )
    )
)]
#[instrument(skip(state, payload))]
pub async fn create_thought(
    State(state): State<AppState>,
    Path(media_id): Path<String>,
    AppJson(payload): AppJson<CreateThoughtPayload>,
) -> Result<(StatusCode, Json<GameThought>), ApiError> {
    let thought = ReviewService::create_thought(
        &state.db,
        &media_id,
        payload.title,
        payload.content,
        payload.category,
        payload.playtime_minutes,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(thought)))
}

#[utoipa::path(
    put,
    path = "/media/{media_id}/thoughts/{thought_id}",
    params(
        ("media_id" = String, Path, description = "Media ID"),
        ("thought_id" = String, Path, description = "Thought ID"),
    ),
    request_body = UpdateThoughtPayload,
    responses(
        (
            status = 200,
            body = GameThought,
            description = "Updated thought"
        )
    )
)]
#[instrument(skip(state, payload))]
pub async fn update_thought(
    State(state): State<AppState>,
    Path((_media_id, thought_id)): Path<(String, String)>,
    AppJson(payload): AppJson<UpdateThoughtPayload>,
) -> Result<Json<GameThought>, ApiError> {
    let thought = ReviewService::update_thought(
        &state.db,
        &thought_id,
        payload.title,
        payload.content,
        payload.category,
    )
    .await?;

    Ok(Json(thought))
}

#[utoipa::path(
    delete,
    path = "/media/{media_id}/thoughts/{thought_id}",
    params(
        ("media_id" = String, Path, description = "Media ID"),
        ("thought_id" = String, Path, description = "Thought ID"),
    ),
    responses(
        (
            status = 204,
            description = "Thought successfully deleted"
        )
    )
)]
#[instrument(skip(state))]
pub async fn delete_thought(
    State(state): State<AppState>,
    Path((_media_id, thought_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    ReviewService::delete_thought(&state.db, &thought_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

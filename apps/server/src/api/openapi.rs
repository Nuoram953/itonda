use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::media::handlers::get_media
    ),
    components(
        schemas(
            crate::api::media::schemas::MediaResponse
        )
    ),
    tags(
        (name = "media", description = "Media management")
    )
)]
pub struct ApiDoc;

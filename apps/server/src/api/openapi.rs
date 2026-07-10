use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::media::handlers::get_media,
        crate::api::media::handlers::import_media
    ),
    components(
        schemas(
            crate::api::media::schemas::MediaResponse
        )
    ),
    tags(
        (name = "media", description = "Media management")
    ),
    servers(
        (url="/api/v1")
        )

)]
pub struct ApiDoc;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::media::handlers::get_media,
        crate::api::media::handlers::get_media_by_id,
        crate::api::media::handlers::import_media,
        crate::api::media::handlers::refresh,
        crate::api::media::handlers::launch_media,
        crate::api::assets::handlers::get_asset
    ),
    components(
        schemas(
            crate::api::media::schemas::MediaResponse
        )
    ),
    tags(
        (name = "media", description = "Media management"),
        (name = "assets", description = "Assets management")
    ),
    servers(
        (url="/api/v1")
        )

)]
pub struct ApiDoc;

use axum::Router;
use utoipa_swagger_ui::SwaggerUi;

mod api;

use itonda_database::connection;
use api::openapi::ApiDoc;
use utoipa::OpenApi;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(api::router())
        .merge(
            SwaggerUi::new("/swagger")
                .url("/api-docs/openapi.json", ApiDoc::openapi())
        );

    let listener = tokio::net::TcpListener::bind(
        "0.0.0.0:3005"
    )
    .await
    .unwrap();

    let pool = connection::connect(
    "sqlite://itonda.db").await;

    connection::migrate(&pool).await.unwrap();

    println!("Server running on http://localhost:3005");

    axum::serve(listener, app)
        .await
        .unwrap();

}

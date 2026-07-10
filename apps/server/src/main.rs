use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello from itonda" }));

    let listener = tokio::net::TcpListener::bind(
        "0.0.0.0:3000"
    )
    .await
    .unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}

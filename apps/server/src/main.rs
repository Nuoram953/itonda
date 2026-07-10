use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello from itonda" }));

    let listener = tokio::net::TcpListener::bind(
        "0.0.0.0:3005"
    )
    .await
    .unwrap();

    println!("Server running on http://localhost:3005");

    axum::serve(listener, app)
        .await
        .unwrap();
}

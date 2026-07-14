use axum::body::to_bytes;
use axum::response::Response;
use serde::de::DeserializeOwned;

pub async fn json<T>(response: Response) -> T
where
    T: DeserializeOwned,
{
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();

    serde_json::from_slice(&body).unwrap()
}

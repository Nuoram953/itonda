use crate::sources::duckduckgo::{
    client::DuckDuckGoImageClient,
    models::DuckDuckGoSearchResponse,
};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, query_param},
};

#[test]
fn test_deserialize_duckduckgo_search_response() {
    let json = r#"{
        "results": [
            {
                "image": "https://example.com/screenshot.jpg",
                "title": "Gears of War Gameplay",
                "width": 1920,
                "height": 1080
            }
        ]
    }"#;

    let res: DuckDuckGoSearchResponse = serde_json::from_str(json).unwrap();
    assert_eq!(res.results.len(), 1);
    assert_eq!(res.results[0].image, "https://example.com/screenshot.jpg");
    assert_eq!(res.results[0].title, "Gears of War Gameplay");
    assert_eq!(res.results[0].width, 1920);
    assert_eq!(res.results[0].height, 1080);
}

#[tokio::test]
async fn test_duckduckgo_client_search() {
    let server = MockServer::start().await;

    // Mock VQD HTML response
    Mock::given(method("GET"))
        .and(query_param("iax", "images"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"<html><script>vqd="3-123456789";</script></html>"#),
        )
        .mount(&server)
        .await;

    // Mock search JSON response
    Mock::given(method("GET"))
        .and(query_param("vqd", "3-123456789"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "results": [
                {
                    "image": "https://images.com/ddg_result.jpg",
                    "title": "Screenshot",
                    "width": 800,
                    "height": 600
                }
            ]
        })))
        .mount(&server)
        .await;

    let client = DuckDuckGoImageClient::new().with_urls(server.uri(), server.uri());
    let image_url = client.search_first_image("test query").await.unwrap();

    assert_eq!(image_url, Some("https://images.com/ddg_result.jpg".into()));
}

#[tokio::test]
async fn test_duckduckgo_screenshot_fetcher() {
    use crate::assets::{traits::ScreenshotFetcher, types::AssetType};
    use crate::sources::duckduckgo::DuckDuckGo;

    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(query_param("iax", "images"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"<html><script>vqd="token-123";</script></html>"#),
        )
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(query_param("vqd", "token-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "results": [
                {
                    "image": "https://images.com/screenshot.jpg",
                    "title": "Gears of War Gameplay Screenshot",
                    "width": 1920,
                    "height": 1080
                }
            ]
        })))
        .mount(&server)
        .await;

    let client = DuckDuckGoImageClient::new().with_urls(server.uri(), server.uri());
    let ddg = DuckDuckGo::with_client(client);

    let asset = ddg
        .discover_screenshot(None, None, None, "Gears of War")
        .await
        .unwrap();

    assert!(asset.is_some());
    let asset = asset.unwrap();
    assert_eq!(asset.asset_type, AssetType::Screenshot);
    assert_eq!(asset.url, "https://images.com/screenshot.jpg");
}

#[tokio::test]
async fn test_duckduckgo_client_rate_limiter() {
    use crate::http::RateLimiter;

    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(query_param("iax", "images"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"<html><script>vqd="rate-limit-token";</script></html>"#),
        )
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(query_param("vqd", "rate-limit-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "results": [
                {
                    "image": "https://images.com/img.jpg",
                    "title": "Title",
                    "width": 100,
                    "height": 100
                }
            ]
        })))
        .mount(&server)
        .await;

    let client = DuckDuckGoImageClient::with_rate_limiter(RateLimiter::new(2, 1.0))
        .with_urls(server.uri(), server.uri());

    let res = client.search_first_image("query").await.unwrap();
    assert_eq!(res, Some("https://images.com/img.jpg".into()));
}

#[test]
fn test_score_pillar_image_prefers_widescreen_and_penalizes_covers() {
    use crate::sources::duckduckgo::{
        models::DuckDuckGoImageResult,
        score_pillar_image,
    };

    let cover_art = DuckDuckGoImageResult {
        image: "https://example.com/cover.jpg".into(),
        title: "Gears of War 3 DVD Cover Art Boxart".into(),
        width: 640,
        height: 960, // portrait ratio 0.66
    };

    let gameplay_screenshot = DuckDuckGoImageResult {
        image: "https://example.com/screenshot.jpg".into(),
        title: "Gears of War 3 Active Reload Gameplay Screenshot".into(),
        width: 1920,
        height: 1080, // widescreen 16:9 ratio 1.77
    };

    let score_cover = score_pillar_image(&cover_art, "Gears of War 3", "Active Reload");
    let score_screenshot = score_pillar_image(&gameplay_screenshot, "Gears of War 3", "Active Reload");

    assert!(score_screenshot > score_cover);
    assert!(score_screenshot > 80);
    assert!(score_cover < 0);
}

#[tokio::test]
async fn test_duckduckgo_pillar_screenshot_fetcher_selects_best_scored_image() {
    use crate::assets::{traits::PillarScreenshotFetcher, types::AssetType};
    use crate::sources::duckduckgo::DuckDuckGo;

    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(query_param("iax", "images"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"<html><script>vqd="token-456";</script></html>"#),
        )
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(query_param("vqd", "token-456"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "results": [
                {
                    "image": "https://example.com/cover_box.jpg",
                    "title": "Gears of War 3 Retail Box Art Cover",
                    "width": 600,
                    "height": 900
                },
                {
                    "image": "https://images.com/true_gameplay.jpg",
                    "title": "Gears of War 3 Gameplay Screenshot",
                    "width": 1920,
                    "height": 1080
                }
            ]
        })))
        .mount(&server)
        .await;

    let client = DuckDuckGoImageClient::new().with_urls(server.uri(), server.uri());
    let ddg = DuckDuckGo::with_client(client);

    let asset = ddg
        .discover_pillar_screenshot(None, None, None, "Gears of War 3", "Active Reload")
        .await
        .unwrap();

    assert!(asset.is_some());
    let asset = asset.unwrap();
    assert_eq!(asset.asset_type, AssetType::Screenshot);
    assert_eq!(asset.url, "https://images.com/true_gameplay.jpg");
}

#[test]
fn test_generate_pillar_queries() {
    use crate::sources::duckduckgo::generate_pillar_queries;

    // Special character ('&') generates composite and segment queries
    let compound_queries = generate_pillar_queries("Gears of War 3", "Experience & Medals");
    assert_eq!(compound_queries.len(), 3);
    assert!(compound_queries.contains(&"Gears of War 3 Experience Medals screenshot".to_string()));
    assert!(compound_queries.contains(&"Gears of War 3 Experience screenshot".to_string()));
    assert!(compound_queries.contains(&"Gears of War 3 Medals screenshot".to_string()));

    // Space-only generates a single combined query
    let single_queries = generate_pillar_queries("Gears of War 3", "Active Reload");
    assert_eq!(single_queries.len(), 1);
    assert_eq!(single_queries[0], "Gears of War 3 Active Reload screenshot");
}

#[test]
fn test_score_pillar_image_requires_game_title() {
    use crate::sources::duckduckgo::{
        models::DuckDuckGoImageResult,
        score_pillar_image,
    };

    // Candidate 1: Has game title in name/URL, but only 1 pillar word
    let with_game_title = DuckDuckGoImageResult {
        image: "https://cdn.mobygames.com/screenshots/gears-of-war-3-medals.jpg".into(),
        title: "Gears of War 3 Medals".into(),
        width: 1280,
        height: 720,
    };

    // Candidate 2: Generic image with both pillar words, but no game title anywhere
    let without_game_title = DuckDuckGoImageResult {
        image: "https://example.com/screenshots/experience-and-medals-guide.jpg".into(),
        title: "Experience and Medals Guide".into(),
        width: 1920,
        height: 1080,
    };

    let score1 = score_pillar_image(&with_game_title, "Gears of War 3", "Experience & Medals");
    let score2 = score_pillar_image(&without_game_title, "Gears of War 3", "Experience & Medals");

    assert!(score1 > score2);
    assert!(score1 > 100);
    assert!(score2 < 50);
}

#[tokio::test]
async fn test_duckduckgo_client_retries_on_403_and_succeeds() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(query_param("iax", "images"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"<html><script>vqd="token-retry";</script></html>"#),
        )
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(query_param("vqd", "token-retry"))
        .respond_with(ResponseTemplate::new(403))
        .up_to_n_times(1)
        .mount(&server)
        .await;


    Mock::given(method("GET"))
        .and(query_param("vqd", "token-retry"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "results": [
                {
                    "image": "https://images.com/recovered.jpg",
                    "title": "Recovered Screenshot",
                    "width": 1920,
                    "height": 1080
                }
            ]
        })))
        .mount(&server)
        .await;

    let client = DuckDuckGoImageClient::new().with_urls(server.uri(), server.uri());
    let res = client.search_images("retry query").await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].image, "https://images.com/recovered.jpg");
}

#[tokio::test]
async fn test_duckduckgo_client_returns_forbidden_after_retries_exhausted() {
    use crate::sources::duckduckgo::models::DuckDuckGoError;

    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(query_param("iax", "images"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"<html><script>vqd="token-403";</script></html>"#),
        )
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(query_param("vqd", "token-403"))
        .respond_with(ResponseTemplate::new(403))
        .mount(&server)
        .await;

    let client = DuckDuckGoImageClient::new().with_urls(server.uri(), server.uri());
    let res = client.search_images("forbidden query").await;
    assert!(matches!(res, Err(DuckDuckGoError::Forbidden)));
}

#[tokio::test]
async fn test_duckduckgo_client_returns_rate_limited_on_429() {
    use crate::sources::duckduckgo::models::DuckDuckGoError;

    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(query_param("iax", "images"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"<html><script>vqd="token-429";</script></html>"#),
        )
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(query_param("vqd", "token-429"))
        .respond_with(ResponseTemplate::new(429))
        .mount(&server)
        .await;

    let client = DuckDuckGoImageClient::new().with_urls(server.uri(), server.uri());
    let res = client.search_images("rate limited query").await;
    assert!(matches!(res, Err(DuckDuckGoError::RateLimited)));
}

#[tokio::test]
#[ignore]
async fn test_live_duckduckgo_real_fetch() {
    let client = DuckDuckGoImageClient::new();
    let res = client.search_images("Hollow Knight gameplay screenshot").await;
    assert!(res.is_ok(), "Failed to search images: {:?}", res.err());
    let images = res.unwrap();
    assert!(!images.is_empty(), "Expected at least one image result");
    eprintln!("Successfully fetched {} images from DuckDuckGo", images.len());
}



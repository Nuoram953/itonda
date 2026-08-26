use crate::{
    media::types::MediaType,
    metadata::{
        models::{GeneralMetadata, MetadataProviderId},
        the_internet_game_database::{
            TheInternetGameDatabase,
            models::{
                Company, GetExternalGameResponse, GetGameResponse, GetInvolvedCompanyResponse,
                IgdbNamedItem, Screenshot, TwitchTokenResponse,
            },
        },
        traits::MetadataFetcher,
    },
};

#[test]
fn test_deserialize_twitch_token_response() {
    let json = r#"{
        "access_token": "mock_access_token_12345",
        "expires_in": 5184000,
        "token_type": "bearer"
    }"#;

    let res: TwitchTokenResponse = serde_json::from_str(json).unwrap();
    assert_eq!(res.access_token, "mock_access_token_12345");
    assert_eq!(res.expires_in, 5184000);
    assert_eq!(res.token_type, "bearer");
}

#[test]
fn test_deserialize_external_game_response() {
    let json = r#"[
        {
            "id": 1001,
            "game": 1942
        }
    ]"#;

    let res: Vec<GetExternalGameResponse> = serde_json::from_str(json).unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].id, 1001);
    assert_eq!(res[0].game, 1942);
}

#[test]
fn test_deserialize_involved_companies_response() {
    let json = r#"[
        {
            "id": 501,
            "company": {
                "id": 101,
                "name": "CD Projekt RED"
            },
            "developer": true,
            "publisher": true
        }
    ]"#;

    let res: Vec<GetInvolvedCompanyResponse> = serde_json::from_str(json).unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].id, 501);
    assert!(res[0].developer);
    assert!(res[0].publisher);
    assert_eq!(res[0].company.name, "CD Projekt RED");
}

#[test]
fn test_deserialize_game_response() {
    let json = r#"[
        {
            "id": 1942,
            "name": "The Witcher 3: Wild Hunt",
            "summary": "An open world RPG.",
            "storyline": "Geralt searches for Ciri.",
            "first_release_date": 1431993600,
            "genres": [
                { "id": 12, "name": "Role-playing (RPG)" }
            ],
            "platforms": [
                { "id": 6, "name": "PC (Microsoft Windows)" }
            ],
            "themes": [
                { "id": 1, "name": "Action" },
                { "id": 17, "name": "Fantasy" }
            ],
            "game_modes": [
                { "id": 1, "name": "Single player" }
            ],
            "screenshots": [
                { "url": "//images.igdb.com/igdb/image/upload/t_thumb/co123.jpg" }
            ],
            "collections": [
                { "id": 72, "name": "The Witcher" }
            ],
            "involved_companies": [
                {
                    "id": 501,
                    "company": {
                        "id": 101,
                        "name": "CD Projekt RED"
                    },
                    "developer": true,
                    "publisher": true
                }
            ],
            "url": "https://www.igdb.com/games/the-witcher-3-wild-hunt"
        }
    ]"#;

    let res: Vec<GetGameResponse> = serde_json::from_str(json).unwrap();
    assert_eq!(res.len(), 1);
    let game = &res[0];
    assert_eq!(game.id, 1942);
    assert_eq!(game.name, "The Witcher 3: Wild Hunt");
    assert_eq!(game.genres.len(), 1);
    assert_eq!(game.genres[0].name.as_deref(), Some("Role-playing (RPG)"));
    assert_eq!(game.platforms.len(), 1);
    assert_eq!(
        game.platforms[0].name.as_deref(),
        Some("PC (Microsoft Windows)")
    );
    assert_eq!(game.collections.len(), 1);
    assert_eq!(game.collections[0].name.as_deref(), Some("The Witcher"));
    assert_eq!(game.involved_companies.len(), 1);
    assert_eq!(game.involved_companies[0].company.name, "CD Projekt RED");
    assert!(game.involved_companies[0].developer);
}

#[test]
fn test_into_general_metadata() {
    let company = Company {
        id: 101,
        name: "CD Projekt RED".into(),
    };

    let involved_companies = vec![GetInvolvedCompanyResponse {
        id: 501,
        company,
        developer: true,
        publisher: true,
    }];

    let game = GetGameResponse {
        id: 1942,
        name: "The Witcher 3: Wild Hunt".into(),
        first_release_date: Some(1431993600),
        franchises: vec![],
        game_modes: vec![IgdbNamedItem {
            id: 1,
            name: Some("Single player".into()),
        }],
        genres: vec![IgdbNamedItem {
            id: 12,
            name: Some("Role-playing (RPG)".into()),
        }],
        platforms: vec![IgdbNamedItem {
            id: 6,
            name: Some("PC (Microsoft Windows)".into()),
        }],
        themes: vec![IgdbNamedItem {
            id: 17,
            name: Some("Fantasy".into()),
        }],
        screenshots: vec![Screenshot {
            url: "//images.igdb.com/screenshot1.jpg".into(),
        }],
        involved_companies,
        collections: vec![IgdbNamedItem {
            id: 72,
            name: Some("The Witcher".into()),
        }],
        parent_game: None,
        release_dates: vec![],
        slug: Some("the-witcher-3-wild-hunt".into()),
        storyline: Some("Geralt searches for Ciri.".into()),
        summary: Some("An open world RPG.".into()),
        tags: vec![],
        url: Some("https://www.igdb.com/games/the-witcher-3-wild-hunt".into()),
    };

    let metadata = game.into_general_metadata();
    let GeneralMetadata::Game(game_meta) = metadata;

    assert_eq!(
        game_meta.common.description.as_deref(),
        Some("Geralt searches for Ciri.")
    );
    assert_eq!(
        game_meta.common.summary.as_deref(),
        Some("An open world RPG.")
    );
    assert_eq!(game_meta.common.release_date, Some(1431993600));
    assert_eq!(game_meta.common.genres, vec!["Role-playing (RPG)"]);
    assert_eq!(game_meta.common.tags, vec!["Fantasy", "Single player"]);
    assert_eq!(game_meta.developers, vec!["CD Projekt RED"]);
    assert_eq!(game_meta.publishers, vec!["CD Projekt RED"]);
    assert_eq!(game_meta.platforms, vec!["PC (Microsoft Windows)"]);
    assert_eq!(game_meta.series.as_deref(), Some("The Witcher"));
}

#[test]
fn test_metadata_fetcher_trait() {
    let igdb = TheInternetGameDatabase::new("client_id".into(), "client_secret".into());
    assert_eq!(igdb.id(), MetadataProviderId::TheInternetGameDatabase);
    assert_eq!(igdb.name(), "IGDB");
    assert!(igdb.supports_media_type(MediaType::Game));
    assert!(!igdb.supports_media_type(MediaType::Movie));
    assert!(!igdb.supports_media_type(MediaType::TvShow));
}

#[tokio::test]
async fn test_client_token_lock_flow() {
    use crate::metadata::the_internet_game_database::models::CachedToken;
    use std::time::{Duration, Instant};

    let igdb = TheInternetGameDatabase::new("client_id".into(), "client_secret".into());
    {
        let mut write_guard = igdb.client().token_cache.write().await;
        *write_guard = Some(CachedToken {
            token: "preloaded_valid_token".into(),
            expires_at: Instant::now() + Duration::from_secs(3600),
        });
    }

    let token = igdb.client().get_token().await.unwrap();
    assert_eq!(token, "preloaded_valid_token");
}

#[tokio::test]
async fn test_rate_limiter_concurrency() {
    use crate::http::RateLimiter;

    let limiter = RateLimiter::new(8, 100.0);
    assert_eq!(limiter.available_concurrency_slots(), 8);

    let mut permits = Vec::new();
    for _ in 0..8 {
        permits.push(limiter.acquire().await);
    }
    assert_eq!(limiter.available_concurrency_slots(), 0);

    permits.pop();
    assert_eq!(limiter.available_concurrency_slots(), 1);
}

#[tokio::test]
async fn test_rate_limiter_pacing() {
    use crate::http::RateLimiter;
    use std::time::Instant;

    let limiter = RateLimiter::new(8, 4.0);

    let start = Instant::now();
    for _ in 0..4 {
        let _permit = limiter.acquire().await;
    }
    let burst_elapsed = start.elapsed();
    assert!(
        burst_elapsed.as_millis() < 100,
        "First 4 requests should burst immediately"
    );

    let _fifth_permit = limiter.acquire().await;
    let total_elapsed = start.elapsed();
    assert!(
        total_elapsed.as_millis() >= 200,
        "5th request should be rate-limited by ~250ms, elapsed: {:?}",
        total_elapsed
    );
}

#[tokio::test]
async fn test_client_surfaces_http_error() {
    use crate::http::RateLimiter;
    use crate::metadata::the_internet_game_database::{
        client::TheInternetGameDatabaseClient, models::CachedToken,
    };
    use std::time::{Duration, Instant};
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/games"))
        .respond_with(ResponseTemplate::new(429).set_body_string("Too Many Requests"))
        .mount(&server)
        .await;

    let client = TheInternetGameDatabaseClient::with_rate_limiter(
        "test_client_id",
        "test_client_secret",
        RateLimiter::new(8, 100.0),
    )
    .with_base_url(format!("{}/", server.uri()));

    {
        let mut write_guard = client.token_cache.write().await;
        *write_guard = Some(CachedToken {
            token: "valid_token".into(),
            expires_at: Instant::now() + Duration::from_secs(3600),
        });
    }

    let result = client.get_game(1942).await;
    assert!(result.is_err());
    let err_str = result.err().unwrap().to_string();
    assert!(err_str.contains("429") || err_str.contains("Too Many Requests"));
}

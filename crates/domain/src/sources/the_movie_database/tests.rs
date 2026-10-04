use crate::{
    assets::types::AssetType,
    media::types::MediaType,
    metadata::traits::MediaSearcher,
    sources::the_movie_database::{
        TheMovieDatabase,
        models::{
            TmdbImageItem, TmdbImagesResponse, TmdbKeywordSearchResponse, TmdbMovieResult,
            TmdbMovieSearchResponse, TmdbMultiSearchResponse, TmdbTvResult, TmdbTvSearchResponse,
        },
    },
};

#[test]
fn test_deserialize_movie_search_response() {
    let json = r#"{
        "page": 1,
        "results": [
            {
                "id": 550,
                "title": "Fight Club",
                "poster_path": "/pB8BM7pdSp6B6Ih7QZ4DrQ3PmJK.jpg"
            }
        ],
        "total_pages": 1,
        "total_results": 1
    }"#;

    let res: TmdbMovieSearchResponse = serde_json::from_str(json).unwrap();
    assert_eq!(res.results.len(), 1);
    assert_eq!(res.results[0].id, 550);
    assert_eq!(res.results[0].title.as_deref(), Some("Fight Club"));
    assert_eq!(
        res.results[0].poster_path.as_deref(),
        Some("/pB8BM7pdSp6B6Ih7QZ4DrQ3PmJK.jpg")
    );
}

#[test]
fn test_deserialize_tv_search_response() {
    let json = r#"{
        "page": 1,
        "results": [
            {
                "id": 1396,
                "name": "Breaking Bad",
                "poster_path": "/ztEaY1wioNo1ZaDSuio9R8egi2b.jpg"
            }
        ],
        "total_pages": 1,
        "total_results": 1
    }"#;

    let res: TmdbTvSearchResponse = serde_json::from_str(json).unwrap();
    assert_eq!(res.results.len(), 1);
    assert_eq!(res.results[0].id, 1396);
    assert_eq!(res.results[0].name.as_deref(), Some("Breaking Bad"));
    assert_eq!(
        res.results[0].poster_path.as_deref(),
        Some("/ztEaY1wioNo1ZaDSuio9R8egi2b.jpg")
    );
}

#[test]
fn test_deserialize_multi_search_response() {
    let json = r#"{
        "page": 1,
        "results": [
            {
                "id": 1396,
                "media_type": "tv",
                "name": "Breaking Bad",
                "poster_path": "/ztEaY1wioNo1ZaDSuio9R8egi2b.jpg"
            }
        ],
        "total_pages": 1,
        "total_results": 1
    }"#;

    let res: TmdbMultiSearchResponse = serde_json::from_str(json).unwrap();
    assert_eq!(res.results.len(), 1);
    assert_eq!(res.results[0].id, 1396);
    assert_eq!(res.results[0].media_type, "tv");
}

#[test]
fn test_deserialize_keyword_search_response() {
    let json = r#"{
        "page": 1,
        "results": [
            {
                "id": 825,
                "name": "superhero"
            }
        ],
        "total_pages": 1,
        "total_results": 1
    }"#;

    let res: TmdbKeywordSearchResponse = serde_json::from_str(json).unwrap();
    assert_eq!(res.results.len(), 1);
    assert_eq!(res.results[0].id, 825);
    assert_eq!(res.results[0].name, "superhero");
}

#[test]
fn test_deserialize_images_response() {
    let json = r#"{
        "id": 550,
        "backdrops": [],
        "posters": [
            {
                "aspect_ratio": 0.667,
                "height": 1500,
                "width": 1000,
                "file_path": "/pB8BM7pdSp6B6Ih7QZ4DrQ3PmJK.jpg",
                "iso_639_1": "en",
                "vote_average": 5.384,
                "vote_count": 4
            }
        ],
        "logos": []
    }"#;

    let res: TmdbImagesResponse = serde_json::from_str(json).unwrap();
    assert_eq!(res.id, 550);
    assert_eq!(res.posters.len(), 1);
    assert_eq!(res.posters[0].file_path, "/pB8BM7pdSp6B6Ih7QZ4DrQ3PmJK.jpg");
}

#[test]
fn test_into_poster_assets() {
    let images = TmdbImagesResponse {
        id: 550,
        backdrops: vec![],
        posters: vec![TmdbImageItem {
            aspect_ratio: Some(0.667),
            height: Some(1500),
            width: Some(1000),
            file_path: "/pB8BM7pdSp6B6Ih7QZ4DrQ3PmJK.jpg".into(),
            iso_639_1: Some("en".into()),
            vote_average: Some(5.0),
            vote_count: Some(10),
        }],
        logos: vec![],
    };

    let assets = images.into_poster_assets();
    assert_eq!(assets.len(), 1);
    assert_eq!(assets[0].asset_type, AssetType::Poster);
    assert_eq!(
        assets[0].url,
        "https://image.tmdb.org/t/p/original/pB8BM7pdSp6B6Ih7QZ4DrQ3PmJK.jpg"
    );
}

#[test]
fn test_tmdb_media_searcher_trait() {
    let tmdb = TheMovieDatabase::new("api_key".into());
    assert!(MediaSearcher::supports_media_type(&tmdb, MediaType::Movie));
    assert!(MediaSearcher::supports_media_type(&tmdb, MediaType::TvShow));
    assert!(!MediaSearcher::supports_media_type(&tmdb, MediaType::Game));
}

#[test]
fn test_movie_into_search_result() {
    let movie = TmdbMovieResult {
        id: 550,
        title: Some("Fight Club".into()),
        original_title: None,
        poster_path: Some("/pB8BM7pdSp6B6Ih7QZ4DrQ3PmJK.jpg".into()),
        backdrop_path: None,
        overview: Some("An insomniac office worker...".into()),
        release_date: Some("1999-10-15".into()),
    };

    let result = movie.into_search_result();
    assert_eq!(result.external_id, "550");
    assert_eq!(result.title, "Fight Club");
    assert_eq!(result.media_type, MediaType::Movie);
    assert_eq!(result.year, Some(1999));
    assert_eq!(
        result.summary.as_deref(),
        Some("An insomniac office worker...")
    );
    assert_eq!(
        result.cover_url.as_deref(),
        Some("https://image.tmdb.org/t/p/w500/pB8BM7pdSp6B6Ih7QZ4DrQ3PmJK.jpg")
    );
}

#[test]
fn test_tv_into_search_result() {
    let tv = TmdbTvResult {
        id: 1396,
        name: Some("Breaking Bad".into()),
        original_name: None,
        poster_path: Some("/ztEaY1wioNo1ZaDSuio9R8egi2b.jpg".into()),
        backdrop_path: None,
        overview: Some("A high school chemistry teacher...".into()),
        first_air_date: Some("2008-01-20".into()),
    };

    let result = tv.into_search_result();
    assert_eq!(result.external_id, "1396");
    assert_eq!(result.title, "Breaking Bad");
    assert_eq!(result.media_type, MediaType::TvShow);
    assert_eq!(result.year, Some(2008));
    assert_eq!(
        result.summary.as_deref(),
        Some("A high school chemistry teacher...")
    );
    assert_eq!(
        result.cover_url.as_deref(),
        Some("https://image.tmdb.org/t/p/w500/ztEaY1wioNo1ZaDSuio9R8egi2b.jpg")
    );
}

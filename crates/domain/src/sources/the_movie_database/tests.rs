use crate::{
    assets::types::AssetType,
    sources::the_movie_database::models::{
        TmdbImageItem, TmdbImagesResponse, TmdbKeywordSearchResponse, TmdbMovieSearchResponse,
        TmdbMultiSearchResponse, TmdbTvSearchResponse,
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

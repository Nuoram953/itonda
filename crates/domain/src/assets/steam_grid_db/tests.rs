use crate::assets::{
    steam_grid_db::models::{Media, MediaAuthor, MediaResponse},
    types::AssetType,
};

fn media_fixture(url: &str) -> Media {
    Media {
        id: 123,
        url: url.into(),
        score: 10,
        style: "official".into(),
        mime: "image/png".into(),
        thumb: format!("{url}/thumb"),
        author: MediaAuthor {
            name: "Author".into(),
            steam64: "".into(),
            avatar: "avatar".into(),
        },
    }
}

#[test]
fn deserialize_media_response() {
    let json = r#"
    {
        "success": true,
        "page": 1,
        "total": 1,
        "limit": 50,
        "data": [
            {
                "id": 123,
                "url": "https://cdn.steamgriddb.com/grid/123.png",
                "score": 10,
                "style": "official",
                "mime": "image/png",
                "thumb": "https://cdn.steamgriddb.com/thumb/123.png",
                "author": {
                    "name": "Author",
                    "steam64": "123456789",
                    "avatar": "https://avatar.url"
                }
            }
        ]
    }
    "#;

    let response: MediaResponse = serde_json::from_str(json).unwrap();

    assert!(response.success);
    assert_eq!(response.page, 1);
    assert_eq!(response.total, 1);
    assert_eq!(response.limit, 50);

    assert_eq!(response.data.len(), 1);
    assert_eq!(
        response.data[0].url,
        "https://cdn.steamgriddb.com/grid/123.png"
    );
}
#[test]
fn deserialize_empty_media_response() {
    let json = r#"
    {
        "success": true,
        "page": 1,
        "total": 0,
        "limit": 50,
        "data": []
    }
    "#;

    let response: MediaResponse = serde_json::from_str(json).unwrap();

    assert!(response.success);
    assert!(response.data.is_empty());
}

#[test]
fn deserialize_failed_response() {
    let json = r#"
    {
        "success": false,
        "page": 1,
        "total": 0,
        "limit": 50,
        "data": []
    }
    "#;

    let response: MediaResponse = serde_json::from_str(json).unwrap();

    assert!(!response.success);
    assert!(response.data.is_empty());
}

#[test]
fn converts_media_response_into_discovered_assets() {
    let response = MediaResponse {
        success: true,
        page: 1,
        total: 2,
        limit: 50,
        data: vec![
            media_fixture("https://cdn.steamgriddb.com/grid/poster1.png".into()),
            media_fixture("https://cdn.steamgriddb.com/grid/poster2.png".into()),
        ],
    };

    let assets = response.into_assets(AssetType::Poster);

    assert_eq!(assets.len(), 2);

    assert_eq!(assets[0].asset_type, AssetType::Poster);
    assert_eq!(
        assets[0].url,
        "https://cdn.steamgriddb.com/grid/poster1.png"
    );

    assert_eq!(assets[1].asset_type, AssetType::Poster);
    assert_eq!(
        assets[1].url,
        "https://cdn.steamgriddb.com/grid/poster2.png"
    );
}

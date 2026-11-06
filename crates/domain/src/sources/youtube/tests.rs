use crate::{
    media::types::MediaType,
    sources::youtube::{client::YoutubeClient, YoutubeSource},
};

#[test]
fn test_extract_video_id() {
    assert_eq!(
        YoutubeClient::extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
        Some("dQw4w9WgXcQ".to_string())
    );

    assert_eq!(
        YoutubeClient::extract_video_id("https://youtu.be/dQw4w9WgXcQ"),
        Some("dQw4w9WgXcQ".to_string())
    );

    assert_eq!(
        YoutubeClient::extract_video_id("https://www.youtube.com/shorts/dQw4w9WgXcQ"),
        Some("dQw4w9WgXcQ".to_string())
    );

    assert_eq!(
        YoutubeClient::extract_video_id("dQw4w9WgXcQ"),
        Some("dQw4w9WgXcQ".to_string())
    );

    assert_eq!(
        YoutubeClient::extract_video_id("just a search query"),
        None
    );
}

#[test]
fn test_resolve_and_expand_binary_path() {
    let resolved = YoutubeClient::resolve_binary_path(Some("Downloads/yt-dlp"));
    assert!(resolved.ends_with("Downloads/yt-dlp") || resolved == "Downloads/yt-dlp");

    let client = YoutubeClient::with_binary_path("Downloads/yt-dlp");
    assert!(client.binary_path().ends_with("Downloads/yt-dlp") || client.binary_path() == "Downloads/yt-dlp");
}

#[test]
fn test_youtube_source_supports_media_types() {
    let source = YoutubeSource::new();
    assert!(source.supports_media_type(MediaType::Game));
    assert!(source.supports_media_type(MediaType::Movie));
    assert!(source.supports_media_type(MediaType::TvShow));
}

#[test]
fn test_parse_year() {
    assert_eq!(YoutubeSource::parse_year(Some("20241026")), Some(2024));
    assert_eq!(YoutubeSource::parse_year(Some("19990101")), Some(1999));
    assert_eq!(YoutubeSource::parse_year(None), None);
}

#[test]
fn test_parse_release_timestamp() {
    let ts = YoutubeSource::parse_release_timestamp(Some("20241026"));
    assert!(ts.is_some());
    assert!(ts.unwrap() > 0);
}

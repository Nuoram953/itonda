use crate::{
    media::types::MediaType,
    sources::youtube::{
        client::{YoutubeClient, AUDIO_LOUDNORM_FILTER},
        models::YtDlpRawOutput,
        YoutubeSource,
    },
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

#[test]
fn test_yt_dlp_raw_output_with_null_entries() {
    let json = r#"{
        "_type": "playlist",
        "entries": [
            null,
            {
                "id": "dQw4w9WgXcQ",
                "title": "Rick Astley - Never Gonna Give You Up",
                "duration": 213.0,
                "uploader": "RickAstleyVEVO"
            },
            null,
            {
                "id": "abc123xyz89",
                "title": "Long Compilation (3 Hours)",
                "duration": 10800.0,
                "uploader": "OST Channel"
            }
        ]
    }"#;

    let parsed: YtDlpRawOutput = serde_json::from_str(json).expect("should parse null entries successfully");
    let entries = parsed.entries.expect("should have entries");
    assert_eq!(entries.len(), 4);

    let videos: Vec<_> = entries
        .into_iter()
        .flatten()
        .filter_map(|e| e.into_video_info())
        .collect();

    assert_eq!(videos.len(), 2);
    assert_eq!(videos[0].id, "dQw4w9WgXcQ");
    assert_eq!(videos[0].duration, Some(213));
    assert_eq!(videos[1].id, "abc123xyz89");
    assert_eq!(videos[1].duration, Some(10800));

    // Filter by max 5 mins (300 seconds)
    let filtered: Vec<_> = videos
        .into_iter()
        .filter(|v| v.duration.map_or(false, |d| d <= 300))
        .collect();

    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].id, "dQw4w9WgXcQ");
}

#[test]
fn test_audio_loudnorm_filter_constant() {
    assert_eq!(AUDIO_LOUDNORM_FILTER, "loudnorm=I=-14:TP=-1.5:LRA=11");
}

#[tokio::test]
async fn test_normalize_audio_file() {
    // If ffmpeg is not installed on this system, skip test
    if tokio::process::Command::new("ffmpeg")
        .arg("-version")
        .output()
        .await
        .is_err()
    {
        return;
    }

    let temp_dir = tempfile::tempdir().unwrap();
    let audio_file = temp_dir.path().join("test_audio.mp3");

    // Generate a quick 1-second sine wave
    let gen_status = tokio::process::Command::new("ffmpeg")
        .arg("-y")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg("sine=frequency=440:duration=1")
        .arg("-c:a")
        .arg("libmp3lame")
        .arg(&audio_file)
        .status()
        .await;

    if let Ok(status) = gen_status {
        if status.success() {
            let res = YoutubeClient::normalize_audio_file(&audio_file).await;
            assert!(res.is_ok());
            assert!(audio_file.exists());
        }
    }
}

#[tokio::test]
async fn test_normalize_video_file() {
    // If ffmpeg is not installed on this system, skip test
    if tokio::process::Command::new("ffmpeg")
        .arg("-version")
        .output()
        .await
        .is_err()
    {
        return;
    }

    let temp_dir = tempfile::tempdir().unwrap();
    let video_file = temp_dir.path().join("test_video.mp4");

    // Generate a quick 1-second test video with audio
    let gen_status = tokio::process::Command::new("ffmpeg")
        .arg("-y")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg("testsrc=duration=1:size=320x240:rate=15")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg("sine=frequency=440:duration=1")
        .arg("-c:v")
        .arg("libx264")
        .arg("-c:a")
        .arg("aac")
        .arg(&video_file)
        .status()
        .await;

    if let Ok(status) = gen_status {
        if status.success() {
            let res = YoutubeClient::normalize_video_file(&video_file).await;
            assert!(res.is_ok());
            assert!(video_file.exists());
        }
    }
}

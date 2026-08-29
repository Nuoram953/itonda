use std::path::PathBuf;

use reqwest::{Client, Url};
use tokio::fs;
use uuid::Uuid;

use crate::{
    assets::{error::AssetError, types::AssetType},
    storage::path::AppPaths,
};

pub struct AssetDownloader {
    client: Client,
    paths: AppPaths,
}

impl AssetDownloader {
    pub fn new(paths: AppPaths) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            ),
        );
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static(
                "image/avif,image/webp,image/apng,image/svg+xml,image/*,*/*;q=0.8",
            ),
        );
        headers.insert(
            reqwest::header::REFERER,
            reqwest::header::HeaderValue::from_static("https://duckduckgo.com/"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or_else(|_| Client::new());

        Self { client, paths }
    }

    pub async fn download(
        &self,
        media_id: Uuid,
        asset_type: AssetType,
        url: &str,
    ) -> Result<PathBuf, AssetError> {
        let dir = self.paths.media_dir(media_id).join(asset_type.folder());

        fs::create_dir_all(&dir).await?;

        let response = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()
            .map_err(|e| AssetError::Other(e.to_string()))?;

        let bytes = response.bytes().await?;

        if bytes.is_empty() {
            return Err(AssetError::Other("Downloaded asset is empty".into()));
        }

        // Validate image magic bytes and detect actual extension
        let extension = Self::detect_image_extension(&bytes)
            .map(String::from)
            .or_else(|| {
                // Fallback to URL extension if not HTML/text or JSON
                let is_html_or_json = bytes.starts_with(b"<!DOCTYPE")
                    || bytes.starts_with(b"<!doctype")
                    || bytes.starts_with(b"<html")
                    || bytes.starts_with(b"<HTML")
                    || bytes.starts_with(b"<head")
                    || bytes.starts_with(b"<body")
                    || bytes.starts_with(b"{\n")
                    || bytes.starts_with(b"{\"");

                if !is_html_or_json {
                    Some(Self::extension_from_url(url))
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                AssetError::Other(
                    "Downloaded payload is not a valid image (received HTML or corrupt data)".into(),
                )
            })?;

        let path = dir.join(format!("{}_{}.{}", asset_type.folder(), Uuid::new_v4(), extension));

        fs::write(&path, bytes).await?;

        Ok(path)
    }

    pub fn detect_image_extension(bytes: &[u8]) -> Option<&'static str> {
        if bytes.len() < 8 {
            return None;
        }

        // JPEG: 0xFF, 0xD8, 0xFF
        if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
            return Some("jpg");
        }

        // PNG: 0x89, 'P', 'N', 'G', 0x0D, 0x0A, 0x1A, 0x0A
        if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
            return Some("png");
        }

        // WebP: 'R', 'I', 'F', 'F', ... , 'W', 'E', 'B', 'P'
        if bytes.starts_with(b"RIFF") && bytes.len() >= 12 && &bytes[8..12] == b"WEBP" {
            return Some("webp");
        }

        // GIF: 'G', 'I', 'F', '8', '7'/'9', 'a'
        if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
            return Some("gif");
        }

        // AVIF: starts with ftypavif or ftypavis
        if bytes.len() >= 12
            && &bytes[4..8] == b"ftyp"
            && (&bytes[8..12] == b"avif" || &bytes[8..12] == b"avis")
        {
            return Some("avif");
        }

        // SVG: XML or <svg
        if bytes.starts_with(b"<svg")
            || (bytes.starts_with(b"<?xml") && bytes.windows(4).any(|w| w == b"<svg"))
        {
            return Some("svg");
        }

        None
    }

    pub fn extension_from_url(url: &str) -> String {
        Url::parse(url)
            .ok()
            .and_then(|parsed| {
                let path = parsed.path();
                if let Some((_, after_dot)) = path.rsplit_once('.') {
                    let ext = after_dot.split('/').next().unwrap_or(after_dot);
                    let ext = ext.split('?').next().unwrap_or(ext);
                    let ext_clean: String =
                        ext.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
                    if !ext_clean.is_empty() && ext_clean.len() <= 5 {
                        return Some(ext_clean.to_lowercase());
                    }
                }
                None
            })
            .unwrap_or_else(|| "jpg".into())
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    use tempfile::tempdir;
    use uuid::Uuid;

    use crate::{
        assets::{downloader::AssetDownloader, types::AssetType},
        storage::path::AppPaths,
    };

    #[tokio::test]
    async fn downloads_asset_to_media_directory() {
        let server = MockServer::start().await;

        let png_bytes = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x06\x00\x00\x00\x1f\x15\xc4\x89fake image";
        Mock::given(method("GET"))
            .and(path("/poster.png"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(png_bytes.as_slice()))
            .mount(&server)
            .await;

        let temp = tempdir().unwrap();

        let paths = AppPaths {
            config_dir: temp.path().join("config"),
            data_dir: temp.path().join("data"),
        };

        let downloader = AssetDownloader::new(paths.clone());

        let media_id = Uuid::new_v4();

        let path = downloader
            .download(
                media_id,
                AssetType::Poster,
                &format!("{}/poster.png", server.uri()),
            )
            .await
            .unwrap();

        assert!(path.exists());

        assert_eq!(
            path.parent().unwrap(),
            paths
                .data_dir
                .join("media")
                .join(media_id.to_string())
                .join("poster")
        );
        let filename = path.file_name().unwrap().to_str().unwrap();
        assert!(filename.starts_with("poster_"));
        assert!(filename.ends_with(".png"));

        assert_eq!(tokio::fs::read(path).await.unwrap(), png_bytes);
    }

    #[tokio::test]
    async fn creates_asset_directories() {
        let server = MockServer::start().await;

        let webp_bytes = b"RIFF\x20\x00\x00\x00WEBPVP8 \x14\x00\x00\x00fake image payload here";
        Mock::given(method("GET"))
            .and(path("/backdrop.webp"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(webp_bytes.as_slice()))
            .mount(&server)
            .await;

        let temp = tempdir().unwrap();

        let paths = AppPaths {
            config_dir: temp.path().join("config"),
            data_dir: temp.path().join("data"),
        };

        let downloader = AssetDownloader::new(paths.clone());

        let media_id = Uuid::new_v4();

        downloader
            .download(
                media_id,
                AssetType::Backdrop,
                &format!("{}/backdrop.webp", server.uri()),
            )
            .await
            .unwrap();

        assert!(
            paths
                .data_dir
                .join("media")
                .join(media_id.to_string())
                .join("backdrop")
                .exists()
        );
    }

    #[tokio::test]
    async fn rejects_html_error_document() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/error.jpg"))
            .respond_with(ResponseTemplate::new(200).set_body_string("<!DOCTYPE html><html><body>Error</body></html>"))
            .mount(&server)
            .await;

        let temp = tempdir().unwrap();
        let paths = AppPaths {
            config_dir: temp.path().join("config"),
            data_dir: temp.path().join("data"),
        };

        let downloader = AssetDownloader::new(paths);
        let res = downloader.download(Uuid::new_v4(), AssetType::Screenshot, &format!("{}/error.jpg", server.uri())).await;
        assert!(res.is_err());
    }

    #[test]
    fn asset_type_generates_correct_folder() {
        assert_eq!(AssetType::Poster.folder(), "poster");
        assert_eq!(AssetType::Backdrop.folder(), "backdrop");
        assert_eq!(AssetType::Logo.folder(), "logo");
        assert_eq!(AssetType::Icon.folder(), "icon");
    }

    #[test]
    fn extracts_extension_from_url() {
        assert_eq!(
            AssetDownloader::extension_from_url("https://example.com/image.png?size=large"),
            "png"
        );

        assert_eq!(
            AssetDownloader::extension_from_url("https://example.com/image.webp"),
            "webp"
        );

        assert_eq!(
            AssetDownloader::extension_from_url(
                "http://wikia.com/images/DeltaSquad_HiRes.jpg/revision/latest?cb=123"
            ),
            "jpg"
        );

        assert_eq!(
            AssetDownloader::extension_from_url("https://example.com/image"),
            "jpg"
        );
    }
}

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
        Self {
            client: Client::new(),
            paths,
        }
    }

    pub async fn download(
        &self,
        media_id: Uuid,
        asset_type: AssetType,
        url: &str,
    ) -> Result<PathBuf, AssetError> {
        let dir = self.paths.media_dir(media_id).join(asset_type.folder());

        fs::create_dir_all(&dir).await?;

        let response = self.client.get(url).send().await?;

        let bytes = response.bytes().await?;

        let extension = Self::extension_from_url(url);

        let path = dir.join(format!("{}.{}", asset_type.folder(), extension));

        fs::write(&path, bytes).await?;

        Ok(path)
    }

    fn extension_from_url(url: &str) -> String {
        Url::parse(url)
            .ok()
            .and_then(|url| {
                url.path()
                    .rsplit('.')
                    .next()
                    .filter(|ext| *ext != url.path())
                    .map(String::from)
            })
            .unwrap_or_else(|| "img".into())
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

        Mock::given(method("GET"))
            .and(path("/poster.png"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"fake image"))
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
            path,
            paths
                .data_dir
                .join("media")
                .join(media_id.to_string())
                .join("poster")
                .join("poster.png")
        );

        assert_eq!(tokio::fs::read(path).await.unwrap(), b"fake image");
    }

    #[tokio::test]
    async fn creates_asset_directories() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/backdrop.webp"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"fake image"))
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
            AssetDownloader::extension_from_url("https://example.com/image"),
            "img"
        );
    }
}

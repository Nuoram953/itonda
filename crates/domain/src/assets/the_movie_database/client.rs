use reqwest_middleware::ClientWithMiddleware;

use crate::{
    assets::{
        error::AssetError,
        the_movie_database::models::{
            TmdbImagesResponse, TmdbKeywordSearchResponse, TmdbMediaType, TmdbMovieSearchResponse,
            TmdbMultiSearchResponse, TmdbTvSearchResponse,
        },
    },
    http::create_http_client,
    media::types::MediaType,
};

pub struct TheMovieDatabaseClient {
    client: ClientWithMiddleware,
    api_key: String,
    base_url: String,
}

impl TheMovieDatabaseClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: create_http_client(),
            api_key: api_key.into(),
            base_url: "https://api.themoviedb.org/3/".into(),
        }
    }

    pub async fn search_movie(&self, title: &str) -> Result<Option<u64>, AssetError> {
        let response = self
            .client
            .get(format!("{}search/movie", self.base_url))
            .query(&[("api_key", &self.api_key), ("query", &title.to_string())])
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let search = response.json::<TmdbMovieSearchResponse>().await?;
        Ok(search.results.first().map(|m| m.id))
    }

    pub async fn search_tv(&self, title: &str) -> Result<Option<u64>, AssetError> {
        let response = self
            .client
            .get(format!("{}search/tv", self.base_url))
            .query(&[("api_key", &self.api_key), ("query", &title.to_string())])
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let search = response.json::<TmdbTvSearchResponse>().await?;
        Ok(search.results.first().map(|t| t.id))
    }

    pub async fn search_multi(
        &self,
        title: &str,
    ) -> Result<Option<(TmdbMediaType, u64)>, AssetError> {
        let response = self
            .client
            .get(format!("{}search/multi", self.base_url))
            .query(&[("api_key", &self.api_key), ("query", &title.to_string())])
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let search = response.json::<TmdbMultiSearchResponse>().await?;
        for item in search.results {
            if item.media_type == "tv" {
                return Ok(Some((TmdbMediaType::Tv, item.id)));
            } else if item.media_type == "movie" {
                return Ok(Some((TmdbMediaType::Movie, item.id)));
            }
        }

        Ok(None)
    }

    pub async fn search_keyword(&self, title: &str) -> Result<Option<u64>, AssetError> {
        let response = self
            .client
            .get(format!("{}search/keyword", self.base_url))
            .query(&[("api_key", &self.api_key), ("query", &title.to_string())])
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let search = response.json::<TmdbKeywordSearchResponse>().await?;
        Ok(search.results.first().map(|k| k.id))
    }

    pub async fn discover_movie_by_keyword(
        &self,
        keyword_id: u64,
    ) -> Result<Option<u64>, AssetError> {
        let keyword_str = keyword_id.to_string();
        let response = self
            .client
            .get(format!("{}discover/movie", self.base_url))
            .query(&[("api_key", &self.api_key), ("with_keywords", &keyword_str)])
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let search = response.json::<TmdbMovieSearchResponse>().await?;
        Ok(search.results.first().map(|m| m.id))
    }

    pub async fn discover_tv_by_keyword(&self, keyword_id: u64) -> Result<Option<u64>, AssetError> {
        let keyword_str = keyword_id.to_string();
        let response = self
            .client
            .get(format!("{}discover/tv", self.base_url))
            .query(&[("api_key", &self.api_key), ("with_keywords", &keyword_str)])
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let search = response.json::<TmdbTvSearchResponse>().await?;
        Ok(search.results.first().map(|t| t.id))
    }

    pub async fn find_media_id(
        &self,
        media_type: Option<&MediaType>,
        title: &str,
    ) -> Result<Option<(TmdbMediaType, u64)>, AssetError> {
        match media_type {
            Some(MediaType::TvShow) => {
                if let Some(id) = self.search_tv(title).await? {
                    return Ok(Some((TmdbMediaType::Tv, id)));
                }
                if let Some(keyword_id) = self.search_keyword(title).await?
                    && let Some(id) = self.discover_tv_by_keyword(keyword_id).await?
                {
                    return Ok(Some((TmdbMediaType::Tv, id)));
                }
            }
            Some(MediaType::Movie) => {
                if let Some(id) = self.search_movie(title).await? {
                    return Ok(Some((TmdbMediaType::Movie, id)));
                }
                if let Some(keyword_id) = self.search_keyword(title).await?
                    && let Some(id) = self.discover_movie_by_keyword(keyword_id).await?
                {
                    return Ok(Some((TmdbMediaType::Movie, id)));
                }
            }
            _ => {
                if let Some(res) = self.search_multi(title).await? {
                    return Ok(Some(res));
                }

                if let Some(id) = self.search_movie(title).await? {
                    return Ok(Some((TmdbMediaType::Movie, id)));
                }
                if let Some(id) = self.search_tv(title).await? {
                    return Ok(Some((TmdbMediaType::Tv, id)));
                }

                if let Some(keyword_id) = self.search_keyword(title).await? {
                    if let Some(id) = self.discover_movie_by_keyword(keyword_id).await? {
                        return Ok(Some((TmdbMediaType::Movie, id)));
                    }
                    if let Some(id) = self.discover_tv_by_keyword(keyword_id).await? {
                        return Ok(Some((TmdbMediaType::Tv, id)));
                    }
                }
            }
        }

        Ok(None)
    }

    pub async fn get_media_images(
        &self,
        tmdb_type: TmdbMediaType,
        media_id: u64,
    ) -> Result<TmdbImagesResponse, AssetError> {
        let endpoint = match tmdb_type {
            TmdbMediaType::Movie => format!("{}movie/{}/images", self.base_url, media_id),
            TmdbMediaType::Tv => format!("{}tv/{}/images", self.base_url, media_id),
        };

        let response = self
            .client
            .get(endpoint)
            .query(&[("api_key", &self.api_key)])
            .send()
            .await?;

        let images = response.json::<TmdbImagesResponse>().await?;
        Ok(images)
    }
}

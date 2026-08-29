use std::time::Instant;

use serde::Deserialize;

use crate::metadata::models::{CommonMetadata, GameGeneralMetadata, GeneralMetadata};

#[derive(Debug, Deserialize)]
pub struct TwitchTokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
}

#[derive(Clone, Debug)]
pub struct CachedToken {
    pub token: String,
    pub expires_at: Instant,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct GetExternalGameResponse {
    #[serde(default)]
    pub id: u64,
    pub game: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Company {
    #[serde(default)]
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GetInvolvedCompanyResponse {
    #[serde(default)]
    pub id: u64,
    pub company: Company,
    #[serde(default)]
    pub developer: bool,
    #[serde(default)]
    pub publisher: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct IgdbNamedItem {
    pub id: u64,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Screenshot {
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GetSearchResponse {
    pub id: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GetGameResponse {
    pub id: u64,
    pub name: String,
    pub first_release_date: Option<i64>,

    #[serde(default)]
    pub franchises: Vec<IgdbNamedItem>,

    #[serde(default)]
    pub game_modes: Vec<IgdbNamedItem>,
    #[serde(default)]
    pub genres: Vec<IgdbNamedItem>,
    #[serde(default)]
    pub platforms: Vec<IgdbNamedItem>,
    #[serde(default)]
    pub themes: Vec<IgdbNamedItem>,

    #[serde(default)]
    pub screenshots: Vec<Screenshot>,

    #[serde(default)]
    pub involved_companies: Vec<GetInvolvedCompanyResponse>,

    #[serde(default)]
    pub collections: Vec<IgdbNamedItem>,

    pub parent_game: Option<u64>,

    #[serde(default)]
    pub release_dates: Vec<u64>,

    pub slug: Option<String>,
    pub storyline: Option<String>,
    pub summary: Option<String>,

    #[serde(default)]
    pub tags: Vec<i64>,

    pub url: Option<String>,
}

impl GetGameResponse {
    pub fn into_general_metadata(self) -> GeneralMetadata {
        let release_date = self.first_release_date;

        let genres = self
            .genres
            .into_iter()
            .filter_map(|g| g.name)
            .collect::<Vec<_>>();

        let platforms = self
            .platforms
            .into_iter()
            .filter_map(|p| p.name)
            .collect::<Vec<_>>();

        let mut tags = Vec::new();
        for theme in self.themes {
            if let Some(name) = theme.name
                && !tags.contains(&name)
            {
                tags.push(name);
            }
        }
        for mode in self.game_modes {
            if let Some(name) = mode.name
                && !tags.contains(&name)
            {
                tags.push(name);
            }
        }

        let mut developers = Vec::new();
        let mut publishers = Vec::new();

        for company_rel in &self.involved_companies {
            if company_rel.developer && !developers.contains(&company_rel.company.name) {
                developers.push(company_rel.company.name.clone());
            }
            if company_rel.publisher && !publishers.contains(&company_rel.company.name) {
                publishers.push(company_rel.company.name.clone());
            }
        }

        let series = self
            .collections
            .into_iter()
            .find_map(|c| c.name)
            .or_else(|| self.franchises.into_iter().find_map(|f| f.name));

        GeneralMetadata::Game(GameGeneralMetadata {
            common: CommonMetadata {
                description: self.storyline,
                summary: self.summary,
                release_date,
                genres,
                tags,
                external_ids: vec![crate::media::models::MediaExternalId {
                    provider: crate::media::models::ExternalIdProvider::Igdb,
                    external_id: self.id.to_string(),
                }],
            },
            developers,
            publishers,
            platforms,
            series,
        })
    }
}

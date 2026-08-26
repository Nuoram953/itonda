use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{media::types::MediaType, storefronts::models::StorefrontId};

#[derive(Debug, Clone)]
pub struct MetadataQuery<'a> {
    pub title: &'a str,
    pub media_type: MediaType,
    pub storefront: Option<StorefrontId>,
    pub external_id: Option<&'a str>,
    pub force: bool,
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum MetadataProviderId {
    TheInternetGameDatabase,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CommonMetadata {
    pub description: Option<String>,
    pub summary: Option<String>,
    pub release_date: Option<i64>,
    pub genres: Vec<String>,
    pub tags: Vec<String>,
}

impl CommonMetadata {
    pub fn merge(&mut self, source: CommonMetadata) {
        if self.description.is_none() {
            self.description = source.description;
        }
        if self.summary.is_none() {
            self.summary = source.summary;
        }
        if self.release_date.is_none() {
            self.release_date = source.release_date;
        }
        for genre in source.genres {
            if !self.genres.contains(&genre) {
                self.genres.push(genre);
            }
        }
        for tag in source.tags {
            if !self.tags.contains(&tag) {
                self.tags.push(tag);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GameGeneralMetadata {
    pub common: CommonMetadata,
    pub developers: Vec<String>,
    pub publishers: Vec<String>,
    pub platforms: Vec<String>,
    pub series: Option<String>,
}

impl GameGeneralMetadata {
    pub fn merge(&mut self, source: GameGeneralMetadata) {
        self.common.merge(source.common);

        for dev in source.developers {
            if !self.developers.contains(&dev) {
                self.developers.push(dev);
            }
        }
        for publ in source.publishers {
            if !self.publishers.contains(&publ) {
                self.publishers.push(publ);
            }
        }
        for plat in source.platforms {
            if !self.platforms.contains(&plat) {
                self.platforms.push(plat);
            }
        }
        if self.series.is_none() {
            self.series = source.series;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum GeneralMetadata {
    Game(GameGeneralMetadata),
}

impl GeneralMetadata {
    pub fn common(&self) -> &CommonMetadata {
        match self {
            Self::Game(g) => &g.common,
        }
    }

    pub fn common_mut(&mut self) -> &mut CommonMetadata {
        match self {
            Self::Game(g) => &mut g.common,
        }
    }

    pub fn merge(&mut self, other: GeneralMetadata) {
        match (self, other) {
            (Self::Game(target), GeneralMetadata::Game(source)) => {
                target.merge(source);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_common_metadata() {
        let mut base = CommonMetadata {
            description: None,
            summary: Some("base summary".into()),
            release_date: None,
            genres: vec!["Action".into()],
            tags: vec!["Singleplayer".into()],
        };

        let incoming = CommonMetadata {
            description: Some("incoming desc".into()),
            summary: Some("incoming summary".into()),
            release_date: Some(12345678),
            genres: vec!["Action".into(), "RPG".into()],
            tags: vec!["Difficult".into()],
        };

        base.merge(incoming);

        assert_eq!(base.description.as_deref(), Some("incoming desc"));
        assert_eq!(base.summary.as_deref(), Some("base summary")); // preserved
        assert_eq!(base.release_date, Some(12345678));
        assert_eq!(base.genres, vec!["Action", "RPG"]);
        assert_eq!(base.tags, vec!["Singleplayer", "Difficult"]);
    }

    #[test]
    fn test_merge_game_metadata() {
        let mut base = GameGeneralMetadata {
            common: CommonMetadata::default(),
            developers: vec!["Dev 1".into()],
            publishers: vec![],
            platforms: vec!["PC".into()],
            series: None,
        };

        let incoming = GameGeneralMetadata {
            common: CommonMetadata::default(),
            developers: vec!["Dev 1".into(), "Dev 2".into()],
            publishers: vec!["Pub 1".into()],
            platforms: vec!["Switch".into()],
            series: Some("Cool Series".into()),
        };

        base.merge(incoming);

        assert_eq!(base.developers, vec!["Dev 1", "Dev 2"]);
        assert_eq!(base.publishers, vec!["Pub 1"]);
        assert_eq!(base.platforms, vec!["PC", "Switch"]);
        assert_eq!(base.series.as_deref(), Some("Cool Series"));
    }
}

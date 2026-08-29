use crate::metadata::models::GeneralMetadata;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MetadataPolicy {
    #[default]
    Complete,
    FirstOnly,
    All,
}

impl MetadataPolicy {
    pub fn is_satisfied(&self, metadata: &GeneralMetadata) -> bool {
        match self {
            Self::FirstOnly => true,
            Self::All => false,
            Self::Complete => self.is_complete(metadata),
        }
    }

    pub fn is_complete(&self, metadata: &GeneralMetadata) -> bool {
        match metadata {
            GeneralMetadata::Game(game) => {
                let common = &game.common;
                common.description.is_some()
                    && common.summary.is_some()
                    && common.release_date.is_some()
                    && !common.genres.is_empty()
                    && !common.tags.is_empty()
                    && !game.developers.is_empty()
                    && !game.publishers.is_empty()
                    && !game.platforms.is_empty()
                    && game.series.is_some()
                    && !game.pillars.is_empty()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::media::models::GameplayPillar;
    use crate::metadata::models::GameGeneralMetadata;

    #[test]
    fn test_policy_first_only() {
        let policy = MetadataPolicy::FirstOnly;
        let meta = GeneralMetadata::Game(GameGeneralMetadata::default());
        assert!(policy.is_satisfied(&meta));
    }

    #[test]
    fn test_policy_all() {
        let policy = MetadataPolicy::All;
        let mut meta = GameGeneralMetadata::default();
        meta.common.description = Some("desc".into());
        meta.common.summary = Some("sum".into());
        meta.common.release_date = Some(1000);
        meta.common.genres = vec!["genre".into()];
        meta.common.tags = vec!["tag".into()];
        meta.developers = vec!["dev".into()];
        meta.publishers = vec!["pub".into()];
        meta.platforms = vec!["plat".into()];
        meta.series = Some("series".into());
        meta.pillars = vec![GameplayPillar {
            id: "combat".into(),
            title: "Combat".into(),
            description: "desc".into(),
            icon: "combat".into(),
            asset_id: None,
        }];
        let meta = GeneralMetadata::Game(meta);

        assert!(!policy.is_satisfied(&meta));
    }

    #[test]
    fn test_policy_complete() {
        let policy = MetadataPolicy::Complete;
        let mut meta = GameGeneralMetadata::default();
        let gm = GeneralMetadata::Game(meta.clone());
        assert!(!policy.is_satisfied(&gm));

        meta.common.description = Some("desc".into());
        meta.common.summary = Some("sum".into());
        meta.common.release_date = Some(1000);
        meta.common.genres = vec!["genre".into()];
        meta.common.tags = vec!["tag".into()];
        meta.developers = vec!["dev".into()];
        meta.publishers = vec!["pub".into()];
        meta.platforms = vec!["plat".into()];
        meta.series = Some("series".into());
        meta.pillars = vec![GameplayPillar {
            id: "combat".into(),
            title: "Combat".into(),
            description: "desc".into(),
            icon: "combat".into(),
            asset_id: None,
        }];
        let gm = GeneralMetadata::Game(meta);
        assert!(policy.is_satisfied(&gm));
    }
}


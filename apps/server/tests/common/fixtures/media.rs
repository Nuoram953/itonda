use itonda_database::media::{
    MediaInsert, MediaLaunchRow, MediaLaunchUpsert, MediaRow, insert_media, upsert_media_launch,
};
use itonda_domain::media::types::MediaStatus;
use sqlx::SqlitePool;

pub struct MediaFixture {
    pub title: String,
    pub media_type: String,
    pub launch: Option<MediaLaunchFixture>,
}

pub struct MediaLaunchFixture {
    pub agent_id: Option<String>,
    pub name: String,
    pub launch_type: String,
    pub program: String,
    pub arguments: String,
    pub working_directory: Option<String>,
    pub is_default: bool,
    pub enabled: bool,
}

impl Default for MediaFixture {
    fn default() -> Self {
        Self {
            title: "Test Game".into(),
            media_type: "game".into(),
            launch: Some(MediaLaunchFixture::default()),
        }
    }
}

impl Default for MediaLaunchFixture {
    fn default() -> Self {
        Self {
            agent_id: None,
            name: "Default".into(),
            launch_type: "steam".into(),
            program: "steam".into(),
            arguments: r#"["steam://run/9310"]"#.into(),
            working_directory: None,
            is_default: true,
            enabled: true,
        }
    }
}

#[allow(dead_code)]
pub struct MediaFixtureResult {
    pub media: MediaRow,
    pub launch: Option<MediaLaunchRow>,
}

impl MediaFixture {
    pub async fn insert(self, db: &SqlitePool) -> MediaFixtureResult {
        let media = insert_media(
            db,
            MediaInsert {
                title: self.title,
                media_type: self.media_type,
                status_id: MediaStatus::NotStarted.id(),
                ..Default::default()
            },
        )
        .await
        .unwrap();

        let launch = match self.launch {
            Some(launch) => Some(
                upsert_media_launch(
                    db,
                    MediaLaunchUpsert {
                        media_id: media.id.clone(),
                        agent_id: launch.agent_id,
                        name: launch.name,
                        launch_type: launch.launch_type,
                        program: launch.program,
                        arguments: launch.arguments,
                        working_directory: launch.working_directory,
                        is_default: launch.is_default,
                        enabled: launch.enabled,
                    },
                )
                .await
                .unwrap(),
            ),
            None => None,
        };

        MediaFixtureResult {
            media,
            launch: Some(launch.unwrap().value),
        }
    }
}

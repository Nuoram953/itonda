use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YoutubeVideoInfo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub uploader: Option<String>,
    pub duration: Option<u64>,
    pub thumbnail: Option<String>,
    pub upload_date: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct YtDlpRawOutput {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub uploader: Option<String>,
    pub channel: Option<String>,
    pub duration: Option<f64>,
    pub thumbnail: Option<String>,
    pub upload_date: Option<String>,
    pub tags: Option<Vec<String>>,
    pub entries: Option<Vec<Option<YtDlpRawOutput>>>,
}

impl YtDlpRawOutput {
    pub fn into_video_info(self) -> Option<YoutubeVideoInfo> {
        let id = self.id?;
        let title = self.title.unwrap_or_else(|| id.clone());
        let uploader = self.uploader.or(self.channel);
        let duration = self.duration.map(|d| d as u64);
        let tags = self.tags.unwrap_or_default();

        Some(YoutubeVideoInfo {
            id,
            title,
            description: self.description,
            uploader,
            duration,
            thumbnail: self.thumbnail,
            upload_date: self.upload_date,
            tags,
        })
    }
}

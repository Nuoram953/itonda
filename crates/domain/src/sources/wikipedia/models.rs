use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct WikipediaSectionsResponse {
    pub parse: Option<WikipediaParseSectionsPayload>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WikipediaParseSectionsPayload {
    pub title: String,
    pub pageid: Option<u64>,
    #[serde(default)]
    pub sections: Vec<WikipediaSectionItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WikipediaSectionItem {
    #[serde(default)]
    pub toclevel: u32,
    pub level: String,
    pub line: String,
    pub number: String,
    pub index: String,
    pub fromtitle: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WikipediaWikitextResponse {
    pub parse: Option<WikipediaParseWikitextPayload>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WikipediaParseWikitextPayload {
    pub title: String,
    pub pageid: Option<u64>,
    pub wikitext: Option<WikipediaWikitextField>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WikipediaWikitextField {
    #[serde(rename = "*")]
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParsedPillar {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub image_file: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WikipediaImageInfoResponse {
    pub query: Option<WikipediaQueryField>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WikipediaQueryField {
    pub pages: Option<std::collections::HashMap<String, WikipediaPageItem>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WikipediaPageItem {
    pub imageinfo: Option<Vec<WikipediaImageInfoItem>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WikipediaImageInfoItem {
    pub url: Option<String>,
}

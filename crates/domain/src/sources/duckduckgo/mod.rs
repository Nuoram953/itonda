pub mod client;
pub mod models;

#[cfg(test)]
mod tests;

pub use client::DuckDuckGoImageClient;
pub use models::{DuckDuckGoError, DuckDuckGoImageResult, DuckDuckGoSearchResponse};

use async_trait::async_trait;

use crate::{
    assets::{
        error::AssetError,
        models::AssetStoreId,
        traits::{AssetFetcher, PillarScreenshotFetcher, ScreenshotFetcher},
        types::AssetType,
    },
    media::{discovered::DiscoveredAsset, types::MediaType},
    storefronts::models::StorefrontId,
};

pub struct DuckDuckGo {
    client: DuckDuckGoImageClient,
}

impl Default for DuckDuckGo {
    fn default() -> Self {
        Self::new()
    }
}

impl DuckDuckGo {
    pub fn new() -> Self {
        Self {
            client: DuckDuckGoImageClient::new(),
        }
    }

    pub fn with_rate_limiter(rate_limiter: crate::http::RateLimiter) -> Self {
        Self {
            client: DuckDuckGoImageClient::with_rate_limiter(rate_limiter),
        }
    }

    #[cfg(test)]
    pub fn with_client(client: DuckDuckGoImageClient) -> Self {
        Self { client }
    }
}

impl AssetFetcher for DuckDuckGo {
    fn id(&self) -> AssetStoreId {
        AssetStoreId::DuckDuckGo
    }

    fn supports_media_type(&self, media_type: MediaType) -> bool {
        matches!(media_type, MediaType::Game)
    }
}

#[async_trait]
impl ScreenshotFetcher for DuckDuckGo {
    async fn discover_screenshot(
        &self,
        _media_type: Option<MediaType>,
        _storefront: Option<StorefrontId>,
        _external_id: Option<&str>,
        title: &str,
    ) -> Result<Option<DiscoveredAsset>, AssetError> {
        let query = format!("{} gameplay screenshot", title);
        let results = self
            .client
            .search_images(&query)
            .await
            .map_err(|e| AssetError::Other(e.to_string()))?;

        let best = pick_best_image(&results, title, "gameplay");

        Ok(best.map(|url| DiscoveredAsset {
            asset_type: AssetType::Screenshot,
            url,
            provider_external_id: None,
            pillar_id: None,
        }))
    }
}

#[async_trait]
impl PillarScreenshotFetcher for DuckDuckGo {
    async fn discover_pillar_screenshot(
        &self,
        _media_type: Option<MediaType>,
        _storefront: Option<StorefrontId>,
        _external_id: Option<&str>,
        game_title: &str,
        pillar_title: &str,
    ) -> Result<Option<DiscoveredAsset>, AssetError> {
        let queries = generate_pillar_queries(game_title, pillar_title);
        let mut all_results = Vec::new();
        let mut seen_urls = std::collections::HashSet::new();

        for query in queries {
            if let Ok(results) = self.client.search_images(&query).await {
                for r in results {
                    if seen_urls.insert(r.image.clone()) {
                        all_results.push(r);
                    }
                }
            }

            if all_results
                .iter()
                .any(|r| score_pillar_image(r, game_title, pillar_title) >= 120)
            {
                break;
            }
        }

        let best = pick_best_image(&all_results, game_title, pillar_title);

        Ok(best.map(|url| DiscoveredAsset {
            asset_type: AssetType::Screenshot,
            url,
            provider_external_id: None,
            pillar_id: None,
        }))
    }
}

pub(crate) fn generate_pillar_queries(game_title: &str, pillar_title: &str) -> Vec<String> {
    let mut queries = Vec::new();
    let mut seen = std::collections::HashSet::new();

    // 1. Full cleaned query with all words
    let full_clean = sanitize_pillar_title(pillar_title);
    if !full_clean.is_empty() {
        let q = format!("{} {} screenshot", game_title, full_clean);
        seen.insert(q.clone());
        queries.push(q);
    }

    // 2. Only split into sub-queries if special characters (like '&', '/', ',', ':', '+', '-') are present
    let has_special_char = pillar_title
        .chars()
        .any(|c| c == '&' || c == '/' || c == ',' || c == ':' || c == '+' || c == '-');

    if has_special_char {
        for part in pillar_title
            .split(|c: char| c == '&' || c == '/' || c == ',' || c == ':' || c == '+' || c == '-')
        {
            let part_clean = sanitize_pillar_title(part);
            if !part_clean.is_empty() && part_clean != full_clean {
                let q = format!("{} {} screenshot", game_title, part_clean);
                if seen.insert(q.clone()) {
                    queries.push(q);
                }
            }
        }
    }

    queries
}

fn pick_best_image(
    results: &[DuckDuckGoImageResult],
    game_title: &str,
    pillar_title: &str,
) -> Option<String> {
    results
        .iter()
        .map(|r| (score_pillar_image(r, game_title, pillar_title), r))
        .filter(|(score, _)| *score > -1000)
        .max_by_key(|(score, _)| *score)
        .map(|(_, r)| r.image.clone())
        .or_else(|| {
            results
                .iter()
                .find(|r| r.height > 0 && r.width >= r.height)
                .map(|r| r.image.clone())
        })
}

pub(crate) fn sanitize_pillar_title(pillar_title: &str) -> String {
    pillar_title
        .replace('&', " ")
        .replace('/', " ")
        .replace('(', " ")
        .replace(')', " ")
        .replace('"', " ")
        .replace('\'', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn score_pillar_image(
    result: &DuckDuckGoImageResult,
    game_title: &str,
    pillar_title: &str,
) -> i32 {
    let title_lower = result.title.to_lowercase();
    let url_lower = result.image.to_lowercase();
    let w = result.width;
    let h = result.height;

    // 1. Hard rejection: portrait images (box covers, mobile wallpapers)
    if h > 0 && (w as f32 / h as f32) < 1.05 {
        return -9999;
    }

    // 2. Hard rejection: box art, retail covers, wallpapers, soundtracks, unboxings
    let hard_negatives = [
        "box art",
        "boxart",
        "retail cover",
        "dvdcover",
        "dvd cover",
        "full cover",
        "full-cover",
        "/covers/",
        "/cover/",
        "wallpaper",
        "wallpapers",
        "soundtrack",
        "unboxing",
        "packaging material",
        "gamecover",
        "teahub",
        "wallpapercave",
        "wallpaperflare",
        "desktopbackground",
        "disc",
        "guide-cover",
        "cover-art",
        "_cover.",
        "-cover.",
        "poster",
        "-poster.",
        "_poster.",
    ];
    for neg in hard_negatives {
        if title_lower.contains(neg) || url_lower.contains(neg) {
            return -9999;
        }
    }

    let mut score = 0;

    // 3. Game Title Matching (MUST MATCH THE FULL GAME TITLE OR ALL SIGNIFICANT WORDS)
    let game_lower = game_title.to_lowercase();
    let game_stop_words = [
        "of", "the", "a", "an", "in", "on", "at", "to", "for", "with", "and",
    ];
    let game_words: Vec<&str> = game_title
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() > 1 && !game_stop_words.contains(&w.to_lowercase().as_str()))
        .collect();

    let exact_game_in_title = title_lower.contains(&game_lower);
    let exact_game_slug = game_words
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join("-");
    let compact_game_slug = game_words
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join("");

    let url_normalized = url_lower.replace('-', "").replace('_', "");
    let exact_game_in_url = url_lower.contains(&exact_game_slug) || url_normalized.contains(&compact_game_slug);

    let words_in_title = game_words
        .iter()
        .filter(|gw| title_lower.contains(&gw.to_lowercase()))
        .count();
    let words_in_url = game_words
        .iter()
        .filter(|gw| url_lower.contains(&gw.to_lowercase()))
        .count();

    let required_words = if game_words.len() <= 3 {
        game_words.len()
    } else {
        game_words.len().saturating_sub(1)
    };

    let has_all_game_words_title = words_in_title >= required_words;
    let has_all_game_words_url = words_in_url >= required_words;
    let has_game_title = exact_game_in_title
        || exact_game_in_url
        || has_all_game_words_title
        || has_all_game_words_url;

    if !has_game_title {
        // Immediate heavy rejection if the image does not belong to the game
        return -500;
    }

    if exact_game_in_title {
        score += 80;
    } else if has_all_game_words_title {
        score += 50;
    }

    if exact_game_in_url {
        score += 70;
    } else if has_all_game_words_url {
        score += 40;
    }

    // 4. Game Sequel / Number Precision
    let game_numbers: Vec<&str> = game_title
        .split_whitespace()
        .filter(|w| w.chars().all(|c| c.is_ascii_digit()))
        .collect();
    for &gn in &game_numbers {
        for &other_num in &["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
            if other_num != gn {
                let check_pattern = format!(" {} ", other_num);
                let check_dash = format!("-{}-", other_num);
                let check_under = format!("_{}_", other_num);
                if title_lower.contains(&check_pattern)
                    || url_lower.contains(&check_dash)
                    || url_lower.contains(&check_under)
                {
                    score -= 80;
                }
            }
        }
    }

    // 5. Topic Relevance (matching words from pillar title)
    let stop_words = [
        "and", "or", "the", "a", "an", "in", "on", "of", "to", "for", "with", "mode", "system",
        "part", "gameplay", "screenshot",
    ];
    let pillar_words: Vec<&str> = pillar_title
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() > 2 && !stop_words.contains(&w.to_lowercase().as_str()))
        .collect();

    let mut title_matches = 0;
    let mut url_matches = 0;
    for &pw in &pillar_words {
        let pw_lower = pw.to_lowercase();
        if title_lower.contains(&pw_lower) {
            score += 35;
            title_matches += 1;
        }
        if url_lower.contains(&pw_lower) {
            score += 30;
            url_matches += 1;
        }
    }

    if title_matches > 0 || url_matches > 0 {
        score += 35;
    } else if !pillar_words.is_empty() {
        score -= 20;
    }

    // 6. Aspect Ratio scoring (16:9 ~ 1.77 is optimal)
    if h > 0 {
        let ratio = w as f32 / h as f32;
        if (1.4..=2.1).contains(&ratio) {
            score += 35;
        } else if (1.2..=2.4).contains(&ratio) {
            score += 20;
        }
    }

    // 7. Resolution scoring
    let pixels = w * h;
    if pixels >= 1920 * 1080 {
        score += 30;
    } else if pixels >= 1280 * 720 {
        score += 20;
    } else if pixels >= 640 * 360 {
        score += 10;
    } else {
        score -= 20;
    }

    // 8. Penalize YouTube video thumbnails
    if url_lower.contains("ytimg.com")
        || url_lower.contains("youtube.com")
        || title_lower.contains("youtube")
    {
        score -= 40;
    }

    // 9. Guide / Feature / Explainer bonus
    let guide_terms = [
        "guide",
        "explainer",
        "how to",
        "tips",
        "mechanic",
        "breakdown",
        "walkthrough",
    ];
    for gt in guide_terms {
        if title_lower.contains(gt) || url_lower.contains(gt) {
            score += 25;
            break;
        }
    }

    // 10. Quality gaming domains bonus
    let quality_domains = [
        "mobygames",
        "gamerant",
        "ign",
        "gamespot",
        "gamersyde",
        "eurogamer",
        "fandom",
        "wikia",
        "gameinformer",
        "polygon",
        "destructoid",
        "pcgamer",
        "kotaku",
        "gameskinny",
        "thegamer",
        "steam",
    ];
    for domain in quality_domains {
        if url_lower.contains(domain) {
            score += 20;
            break;
        }
    }

    // 11. Direct standard image format bonus
    if url_lower.ends_with(".jpg")
        || url_lower.ends_with(".jpeg")
        || url_lower.ends_with(".png")
        || url_lower.ends_with(".webp")
    {
        score += 5;
    }

    score
}

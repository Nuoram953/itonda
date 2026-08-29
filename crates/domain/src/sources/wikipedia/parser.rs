use regex::Regex;
use std::sync::LazyLock;

use crate::sources::wikipedia::{
    cleaner::clean_wikitext,
    models::ParsedPillar,
};

static RE_PREFIXES: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        Regex::new(r"(?i)^Gameplay focuses heavily on [^;.]+[;.]\s*").unwrap(),
        Regex::new(r#"^(?i:Like (?:its|their|all) predecessors?,?\s+(?:(?:the|an?) )?)(?:(?:''|["])[A-Z0-9][\w: '–-]+(?:''|["])\s+is\s+an?\s+[^,]+(?i:\s+that)\s+|[A-Z0-9][\w: '–-]+\s+is\s+an?\s+[^,]+(?i:\s+that)\s+)?"#).unwrap(),
        Regex::new(r"(?i)^Like (?:its|their|all) predecessors?,?\s*").unwrap(),
        Regex::new(r#"^(?i:New to (?:the (?:series|game|franchise)|(?:''|["])[A-Z0-9][\w: '–-]+(?:''|["])|[A-Z0-9][\w: '–-]+)\s+is\s+(?:the ability to\s+)?)"#).unwrap(),
        Regex::new(r"(?i)^(?:In|Within) the (?:game|series),?\s*").unwrap(),
        Regex::new(r#"^(?i:(?:In|Within))\s+(?:(?:''|["])[A-Z0-9][\w: '–-]+(?:''|["])|[A-Z0-9][\w: '–-]+),?\s*"#).unwrap(),
        Regex::new(r"(?i)^Throughout the game,?\s*").unwrap(),
        Regex::new(r"(?i)^When in combat,?\s*").unwrap(),
        Regex::new(r"(?i)^As with (?:previous|other) (?:games|installations),?\s*").unwrap(),
        Regex::new(r"(?i)^Similar to [^,]+,?\s*").unwrap(),
        Regex::new(r"(?i)^Additionally,?\s*").unwrap(),
        Regex::new(r"(?i)^Furthermore,?\s*").unwrap(),
        Regex::new(r"(?i)^A large (?:form|part|portion) of (?:the )?gameplay consists of\s+").unwrap(),
        Regex::new(r#"^(?:(?:''|["])[A-Z0-9][\w: '–-]+(?:''|["])|[A-Z0-9][\w: '–-]+)\s+(?i:is an? (?:[\w\s-]+ )?(?:game|video game),?\s*(?:where|in which|that)\s+)"#).unwrap(),
    ]
});

static RE_PLAYER_CAN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\b(?:players can now also|the player(?:'s character)? (?:is able to|can))\s+").unwrap());
static RE_PLAYER_MUST: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\bthe player(?:'s character)? must\s+").unwrap());
static RE_PLAYER_EARNS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\bthe player (?:earns|can earn)\s+").unwrap());
static RE_PLAYER_MAINTAINS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\bthe player maintains\s+").unwrap());
static RE_GAME_FEATURES: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\bthe (?:game|system) (?:features|uses)\s+").unwrap());

static RE_FILLERS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        Regex::new(r"(?i),?\s*unlike (?:the )?(?:traditional|standard|other) [^,;.]+(?: in other (?:shooters|games|titles)[^,;.]*)?").unwrap(),
        Regex::new(r"(?i),?\s*as a measure of [^,;.]+").unwrap(),
        Regex::new(r"(?i)\s*\(and [^)]+\)").unwrap(),
        Regex::new(r"(?i)\s*meaning that [^,;.]+").unwrap(),
        Regex::new(r"(?i)\s*\(HUD\)").unwrap(),
    ]
});

static RE_QUOTED_TERMS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#""([^"]{3,30})""#).unwrap());
static RE_SYSTEM_TERMS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\b([a-z0-9]+(?:\s+[a-z0-9]+)?\s+(?:system|mode|tree|trees|minigame|mini-game|network|points|bonuses|operations))\b").unwrap());
static RE_SUBHEADING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^={3,5}\s*([^=]+?)\s*={3,5}$").unwrap());

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Combat,
    Exploration,
    Choices,
    Survival,
    Crafting,
    Stealth,
    Coop,
    Progression,
    Puzzle,
    General,
}

#[derive(Debug, Clone)]
struct CandidateSentence {
    cleaned: String,
    para_idx: usize,
    sent_idx: usize,
    category: Category,
    score: isize,
}

pub fn parse_gameplay_wikitext(raw: &str) -> Vec<ParsedPillar> {
    let cleaned = clean_wikitext(raw);

    let raw_paragraphs: Vec<&str> = cleaned
        .text
        .split("\n\n")
        .map(str::trim)
        .filter(|p| p.len() >= 20)
        .collect();

    if raw_paragraphs.is_empty() {
        return Vec::new();
    }

    let mut para_sentences: Vec<Vec<CandidateSentence>> = Vec::new();
    let mut all_candidates: Vec<CandidateSentence> = Vec::new();

    for (p_idx, para) in raw_paragraphs.iter().enumerate() {
        let sents = split_sentences(para);
        let mut valid_in_para = Vec::new();

        for (s_idx, s) in sents.iter().enumerate() {
            let cs = clean_sentence(s);
            if !cs.is_empty() && !is_pure_boilerplate(&cs) {
                let cat = classify_sentence(&cs, &[]);
                let sc = score_sentence(&cs, cat);
                let item = CandidateSentence {
                    cleaned: cs,
                    para_idx: p_idx,
                    sent_idx: s_idx,
                    category: cat,
                    score: sc,
                };
                valid_in_para.push(item.clone());
                all_candidates.push(item);
            }
        }

        if !valid_in_para.is_empty() {
            para_sentences.push(valid_in_para);
        }
    }

    if all_candidates.is_empty() {
        return Vec::new();
    }

    let num_paras = para_sentences.len();
    let mut selected: Vec<CandidateSentence> = Vec::new();
    let mut selected_cleaned: Vec<String> = Vec::new();
    let mut used_titles: Vec<String> = Vec::new();

    // Balanced Candidate Selection:
    if num_paras >= 3 {
        for p_list in para_sentences.iter().take(4) {
            let mut sorted_p = p_list.clone();
            sorted_p.sort_by(|a, b| b.score.cmp(&a.score));
            for cand in sorted_p {
                let title = derive_title_for_sentence(&cand.cleaned, raw, cand.category, &used_titles);
                if !selected_cleaned.contains(&cand.cleaned) && !used_titles.contains(&title) {
                    selected.push(cand.clone());
                    selected_cleaned.push(cand.cleaned.clone());
                    used_titles.push(title);
                    break;
                }
            }
        }
    } else if num_paras == 2 {
        let high_score_cands: Vec<&CandidateSentence> = all_candidates
            .iter()
            .filter(|c| c.score >= 25)
            .collect();

        if high_score_cands.len() >= 4 {
            let mut sorted_high = high_score_cands;
            sorted_high.sort_by(|a, b| b.score.cmp(&a.score));
            for cand in sorted_high {
                let title = derive_title_for_sentence(&cand.cleaned, raw, cand.category, &used_titles);
                if !selected_cleaned.contains(&cand.cleaned) && !used_titles.contains(&title) {
                    selected.push(cand.clone());
                    selected_cleaned.push(cand.cleaned.clone());
                    used_titles.push(title);
                    if selected.len() >= 4 {
                        break;
                    }
                }
            }
        } else {
            for p_list in &para_sentences {
                let mut sorted_p = p_list.clone();
                sorted_p.sort_by(|a, b| b.score.cmp(&a.score));
                let mut p_count = 0;
                for cand in sorted_p {
                    let title = derive_title_for_sentence(&cand.cleaned, raw, cand.category, &used_titles);
                    if !selected_cleaned.contains(&cand.cleaned) && !used_titles.contains(&title) {
                        selected.push(cand.clone());
                        selected_cleaned.push(cand.cleaned.clone());
                        used_titles.push(title);
                        p_count += 1;
                        if p_count >= 2 {
                            break;
                        }
                    }
                }
            }
        }
    }

    // Fill any remaining slots up to 4 from highest scoring overall candidates (for 1-2 paragraph articles)
    if num_paras < 3 && selected.len() < 4 {
        let mut all_remaining: Vec<CandidateSentence> = all_candidates
            .into_iter()
            .filter(|c| !selected_cleaned.contains(&c.cleaned))
            .collect();
        all_remaining.sort_by(|a, b| b.score.cmp(&a.score));

        for cand in all_remaining {
            let title = derive_title_for_sentence(&cand.cleaned, raw, cand.category, &used_titles);
            if !selected_cleaned.contains(&cand.cleaned) && !used_titles.contains(&title) {
                selected.push(cand.clone());
                selected_cleaned.push(cand.cleaned.clone());
                used_titles.push(title);
                if selected.len() >= 4 {
                    break;
                }
            }
        }
    }

    // Sort selected back to natural narrative order (para_idx, sent_idx)
    selected.sort_by_key(|c| (c.para_idx, c.sent_idx));

    let mut pillars = Vec::new();
    let mut final_titles = Vec::new();

    for (i, sel) in selected.iter().enumerate() {
        let title = derive_title_for_sentence(&sel.cleaned, raw, sel.category, &final_titles);
        final_titles.push(title.clone());

        let description = format_single_sentence_description(&sel.cleaned);
        let icon = category_to_icon(sel.category).to_string();
        let image_file = cleaned.image_files.get(i).cloned();
        let id = format!("{}_{}", category_to_id(sel.category), i + 1);

        pillars.push(ParsedPillar {
            id,
            title,
            description,
            icon,
            image_file,
        });
    }

    pillars
}

fn contains_word(text: &str, word: &str) -> bool {
    if word.contains(' ') || word.contains('-') {
        text.to_lowercase().contains(&word.to_lowercase())
    } else {
        let w_stem = word.trim_end_matches('s');
        text.split(|c: char| !c.is_alphanumeric()).any(|w| {
            w.eq_ignore_ascii_case(word)
                || (!w_stem.is_empty()
                    && w_stem.len() >= 3
                    && w.trim_end_matches('s').eq_ignore_ascii_case(w_stem))
        })
    }
}

fn category_keywords(cat: Category) -> &'static [&'static str] {
    match cat {
        Category::Combat => &[
            "turn-based", "tactics", "tactical", "commands", "action points", "action point",
            "combat", "weapon", "weapons", "shooter", "shooting", "guns", "gun", "melee",
            "sword", "swords", "attack", "damage", "enemies", "enemy", "boss", "cover",
            "reload", "active reload", "magic", "spells", "procedurally generated", "grid",
        ],
        Category::Survival => &[
            "health", "damage", "bleed", "revive", "revived", "permadeath", "permanent death",
            "death", "injuries", "needs", "needs system", "sleep", "eat", "hunger", "thirst",
            "stamina", "vitality", "torches", "raise animals", "raising animals", "barns and coops",
            "barns or coops", "barns", "coops", "livestock",
        ],
        Category::Progression => &[
            "experience", "level", "leveling", "xp", "medals", "ribbons", "attributes", "skills",
            "skill", "perks", "unlock", "stats", "upgrades", "upgrade", "tree", "classes",
            "class", "bundle", "bundles", "community center", "research", "progression system",
            "sabotage", "base of operations",
        ],
        Category::Choices => &[
            "dialogue", "dialogue tree", "dialogue trees", "choice", "choices", "consequence",
            "consequences", "reputation", "decision", "decisions", "quest", "quests", "nonlinear",
            "narrative decisions", "narrative", "converse", "conflicts", "approval", "marry",
            "marriage", "spouse", "relationships",
        ],
        Category::Coop => &[
            "tag enemy", "tagging", "tag", "cooperative", "co-op", "multiplayer", "squad",
            "allies", "ally", "teammates", "team", "bonds", "synergy", "synergies",
            "proximity chat", "voice", "voices", "inaudible", "volume", "walkie talkie", "lobby",
            "lobbies", "dispatch", "superhero",
        ],
        Category::Exploration => &[
            "open world", "explore", "exploration", "map", "world", "dungeon", "dungeons",
            "caves", "mount", "steed", "horse", "vehicle", "traversal", "roam", "travel",
            "navigate", "third-person perspective", "fast travel", "farm types", "farm type",
        ],
        Category::Crafting => &[
            "craft", "crafting", "recipes", "cookbooks", "materials", "forge", "alchemy",
            "potions", "medicines", "repair", "farming", "farm", "plant", "seeds", "harvest",
            "crops", "watering", "farming system", "scythe", "tilling", "smelt", "furnace",
        ],
        Category::Puzzle => &[
            "puzzle", "puzzles", "physics", "environmental", "riddle", "challenges", "codes",
            "miming", "singing", "minigame", "minigames", "hacking", "quick time", "qte", "pathways",
        ],
        Category::Stealth => &[
            "stealth", "hidden", "sneak", "detection", "critical hit", "shadows", "infiltrate",
            "takedown", "backstab", "backstabs",
        ],
        Category::General => &[],
    }
}

fn classify_sentence(sentence: &str, used_categories: &[Category]) -> Category {
    let lower = sentence.to_lowercase();
    let all_categories = [
        Category::Combat,
        Category::Survival,
        Category::Progression,
        Category::Choices,
        Category::Coop,
        Category::Exploration,
        Category::Crafting,
        Category::Puzzle,
        Category::Stealth,
    ];

    let mut best_category = Category::General;
    let mut best_score = 0;

    for cat in all_categories {
        let mut score = 0;
        for kw in category_keywords(cat) {
            if contains_word(&lower, kw) {
                if kw.contains(' ') || kw.contains('-') {
                    score += 2;
                } else {
                    score += 1;
                }
            }
        }
        if score > 0 {
            let adj = if used_categories.contains(&cat) {
                score
            } else {
                score + 2
            };
            if adj > best_score {
                best_score = adj;
                best_category = cat;
            }
        }
    }

    best_category
}

fn score_sentence(s: &str, category: Category) -> isize {
    let lower = s.to_lowercase();
    let mut score = 0;

    let all_categories = [
        Category::Combat,
        Category::Survival,
        Category::Progression,
        Category::Choices,
        Category::Coop,
        Category::Exploration,
        Category::Crafting,
        Category::Puzzle,
        Category::Stealth,
    ];
    for cat in all_categories {
        for kw in category_keywords(cat) {
            if contains_word(&lower, kw) {
                score += 2;
            }
        }
    }

    for kw in category_keywords(category) {
        if contains_word(&lower, kw) {
            score += 4;
        }
    }

    let signature_systems = [
        "system", "mechanic", "mode", "points", "skill tree", "skill trees", "tech tree",
        "minigame", "operations", "synergy", "permadeath", "permanent death", "dialogue tree",
        "active reload", "turn-based", "farm types", "farm type", "farming system",
        "raise animals", "raising animals", "barns and coops", "barns or coops",
        "community center", "bundle", "bundles", "proximity chat", "superhero dispatch",
        "needs system", "tag enemy", "dialogue trees", "reputation", "nonlinear", "voices",
        "inaudible",
    ];
    for sys in signature_systems {
        if contains_word(&lower, sys) {
            score += 18;
            break;
        }
    }

    if lower.contains("reputation is based") || (lower.contains("reputation") && lower.contains("consequences")) {
        score += 8;
    }

    if (lower.contains(" has a ") || lower.contains(" features a ")) && lower.ends_with(" system.") {
        score -= 6;
    }

    if lower.contains('"') {
        score += 3;
    }

    if lower.starts_with("players may develop skills in") || lower.starts_with("skills include") {
        score -= 10;
    }

    if lower.contains("voices") || lower.contains("inaudible") {
        score += 4;
    }

    if s.len() >= 45 && s.len() <= 175 {
        score += 4;
    } else if s.len() < 35 {
        score -= 5;
    } else if s.len() > 190 {
        score -= 2;
    }

    score
}

fn derive_title_for_sentence(
    sentence: &str,
    full_raw: &str,
    category: Category,
    used_titles: &[String],
) -> String {
    let lower = sentence.to_lowercase();

    // 1. Direct semantic mechanics (order by specificity)
    if lower.contains("active reload") {
        return "Active Reload".to_string();
    }
    if lower.contains("crimson omen")
        || (lower.contains("damage") && lower.contains("health"))
        || (lower.contains("damage") && lower.contains("fire"))
    {
        let cand = "Health & Recovery".to_string();
        if !used_titles.contains(&cand) {
            return cand;
        }
    }
    if contains_word(&lower, "bleed")
        || contains_word(&lower, "bleed-out")
        || contains_word(&lower, "revived")
        || contains_word(&lower, "revive")
    {
        let cand = "Revive & Bleed-Out".to_string();
        if !used_titles.contains(&cand) {
            return cand;
        }
    }
    if contains_word(&lower, "tag") || contains_word(&lower, "tagging") {
        let cand = "Squad Tagging & Coordination".to_string();
        if !used_titles.contains(&cand) {
            return cand;
        }
    }
    if lower.contains("experience") || lower.contains("medals") || lower.contains("ribbons") {
        let cand = "Experience & Medals".to_string();
        if !used_titles.contains(&cand) {
            return cand;
        }
    }
    if lower.contains("farm type") || lower.contains("farm types") {
        return "Farm Types".to_string();
    }
    if lower.contains("farming system") || (lower.contains("seasonal seeds") && lower.contains("harvest")) {
        return "Farming System".to_string();
    }
    if lower.contains("raise animals")
        || lower.contains("raising animals")
        || lower.contains("barns and coops")
        || lower.contains("barns or coops")
    {
        return "Raise Animals".to_string();
    }
    if lower.contains("community center") || lower.contains("bundles") || contains_word(&lower, "bundle") {
        return "Community Center Bundles".to_string();
    }
    if lower.contains("superhero dispatch") || (lower.contains("superhero") && lower.contains("dispatch")) {
        return "Superhero Dispatch".to_string();
    }
    if lower.contains("hacking minigame") || (lower.contains("hacking") && lower.contains("minigame")) {
        return "Hacking Minigame".to_string();
    }
    if lower.contains("proximity chat")
        || (lower.contains("voices") && lower.contains("inaudible"))
        || (lower.contains("voice") && lower.contains("proximity"))
    {
        return "Proximity Chat".to_string();
    }
    if contains_word(&lower, "lobby") || contains_word(&lower, "lobbies") {
        return "Multiplayer Lobbies".to_string();
    }
    if contains_word(&lower, "puzzle") || contains_word(&lower, "puzzles") {
        if lower.contains("navigate") || lower.contains("travel") || lower.contains("train") {
            return "Puzzles & Navigation".to_string();
        }
        return "Puzzles & Challenges".to_string();
    }
    if lower.contains("dialogue tree") || lower.contains("dialogue trees") {
        if lower.contains("player's choices affect") || lower.contains("choices affect the story") {
            return "Player's Choice".to_string();
        }
        return "Branched Dialogue".to_string();
    }
    if lower.contains("reputation is based")
        || (lower.contains("choice") && lower.contains("consequence"))
        || (lower.contains("decisions") && lower.contains("consequences"))
    {
        return "Choices & Consequences".to_string();
    }
    if lower.contains("needs system") || (lower.contains("needs") && (lower.contains("sleep") || lower.contains("eat"))) {
        return "Needs System".to_string();
    }
    if lower.contains("quests are intended") || lower.contains("nonlinear") {
        return "Nonlinear Quests".to_string();
    }
    if lower.contains("clothing system") {
        return "Clothing & Equipment".to_string();
    }
    if lower.contains("open world") && (lower.contains("explore") || lower.contains("mount") || lower.contains("steed") || lower.contains("traversal")) {
        return "Open World Exploration".to_string();
    }
    if lower.contains("stealth") || lower.contains("backstab") || lower.contains("backstabs") {
        return "Stealth & Infiltration".to_string();
    }

    // Specific turn-based tactics / Squad / Base mechanics
    if lower.contains("turn-based") || (lower.contains("tactics") && lower.contains("combat") && !lower.contains("cover")) {
        let cand = "Turn-Based Tactical Combat".to_string();
        if !used_titles.contains(&cand) {
            return cand;
        }
    }
    if lower.contains("permadeath") || lower.contains("permanent death") || lower.contains("lasting injuries") {
        let cand = "Permanent Death".to_string();
        if !used_titles.contains(&cand) {
            return cand;
        }
    }
    if lower.contains("base of operations") || lower.contains("the den") || (lower.contains("base") && lower.contains("operations")) || lower.contains("manage a base") {
        let cand = "Base Management & Operations".to_string();
        if !used_titles.contains(&cand) {
            return cand;
        }
    }
    if lower.contains("bonds") || lower.contains("synergy") || lower.contains("synergies") {
        let cand = "Squad Bonds & Tactical Synergy".to_string();
        if !used_titles.contains(&cand) {
            return cand;
        }
    }
    if lower.contains("action point") || lower.contains("action points") {
        let cand = "Action Points & Movement".to_string();
        if !used_titles.contains(&cand) {
            return cand;
        }
    }
    if (lower.contains("squad") || lower.contains("mercenaries")) && (lower.contains("classes") || lower.contains("class") || lower.contains("customize")) {
        let cand = "Squad Customization & Classes".to_string();
        if !used_titles.contains(&cand) {
            return cand;
        }
    } else if lower.contains("classless") || (lower.contains("customise") && lower.contains("skills")) || (lower.contains("customize") && lower.contains("skills")) {
        let cand = "Skills & Customization".to_string();
        if !used_titles.contains(&cand) {
            return cand;
        }
    }
    if lower.contains("sabotage") || (lower.contains("permanent upgrades") && lower.contains("enemies")) {
        let cand = "Progression & Enemy Sabotage".to_string();
        if !used_titles.contains(&cand) {
            return cand;
        }
    }
    if lower.contains("cover") && (lower.contains("combat") || lower.contains("shooter") || lower.contains("survive")) {
        let cand = "Cover Combat & Tactics".to_string();
        if !used_titles.contains(&cand) {
            return cand;
        }
    }
    if lower.contains("weapons") || lower.contains("swords") || lower.contains("weapon") {
        if lower.contains("repair") || lower.contains("distilling") || lower.contains("medicines") {
            let cand = "Crafting & Equipment".to_string();
            if !used_titles.contains(&cand) {
                return cand;
            }
        }
        let cand = "Weapons & Combat".to_string();
        if !used_titles.contains(&cand) {
            return cand;
        }
    }

    // 2. Look for subheadings above paragraph
    if let Some(heading) = find_subheading_for_para(sentence, full_raw) {
        let titled = clean_title_case(&heading);
        if !used_titles.contains(&titled) {
            return titled;
        }
    }

    // 3. Look for quoted terms
    for cap in RE_QUOTED_TERMS.captures_iter(sentence) {
        if let Some(m) = cap.get(1) {
            let term = strip_leading_stop_words(m.as_str().trim());
            if term.len() >= 4 && !term.eq_ignore_ascii_case("hud") {
                let cand = clean_title_case(term);
                if !used_titles.contains(&cand) {
                    return cand;
                }
            }
        }
    }

    // 4. Look for system terms
    for cap in RE_SYSTEM_TERMS.captures_iter(sentence) {
        if let Some(m) = cap.get(1) {
            let term = strip_leading_stop_words(m.as_str().trim());
            if term.len() >= 4 {
                let cand = clean_title_case(term);
                if !used_titles.contains(&cand) {
                    return cand;
                }
            }
        }
    }

    // 5. Category-aware validated defaults
    let fallback = category_default_title(category).to_string();
    if !used_titles.contains(&fallback) {
        return fallback;
    }

    format!("{} (Part {})", fallback, used_titles.len() + 1)
}

fn strip_leading_stop_words(mut term: &str) -> &str {
    let stop_words = ["and", "or", "the", "a", "an", "in", "on", "of", "to", "for", "with", "that", "this", "by", "as", "is"];
    loop {
        let trimmed = term.trim();
        if let Some(first_space) = trimmed.find(' ') {
            let first_word = trimmed[..first_space].to_lowercase();
            if stop_words.contains(&first_word.as_str()) {
                term = trimmed[first_space + 1..].trim_start();
                continue;
            }
        }
        return trimmed;
    }
}

fn format_single_sentence_description(s: &str) -> String {
    let text = s.trim();
    if text.len() <= 175 {
        return ensure_sentence_punctuation(text);
    }

    if let Some(pos) = text[..170].rfind("; ") {
        return ensure_sentence_punctuation(&text[..pos]);
    }
    if let Some(pos) = text[..165].rfind(", where ") {
        if pos >= 50 {
            return ensure_sentence_punctuation(&text[..pos]);
        }
    }
    if let Some(pos) = text[..155].rfind(", ") {
        if pos >= 50 {
            return ensure_sentence_punctuation(&text[..pos]);
        }
    }

    ensure_sentence_punctuation(&format!("{}...", &text[..150].trim_end()))
}

fn ensure_sentence_punctuation(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.ends_with('.') || trimmed.ends_with('!') || trimmed.ends_with('?') || trimmed.ends_with("...") {
        trimmed.to_string()
    } else {
        format!("{trimmed}.")
    }
}

fn find_subheading_for_para(para: &str, full_raw: &str) -> Option<String> {
    let para_snippet = if para.len() > 30 { &para[..30] } else { para };
    if let Some(pos) = full_raw.find(para_snippet) {
        let preceding = &full_raw[..pos];
        if let Some(heading_cap) = RE_SUBHEADING.captures_iter(preceding).last() {
            if let Some(h) = heading_cap.get(1) {
                let heading_text = h.as_str().trim();
                if !heading_text.eq_ignore_ascii_case("gameplay") {
                    return Some(heading_text.to_string());
                }
            }
        }
    }
    None
}

fn clean_title_case(term: &str) -> String {
    let stop_words = ["and", "or", "the", "a", "an", "in", "on", "of", "to", "for", "with", "&"];
    let words: Vec<&str> = term.split_whitespace().collect();
    let mut titled = Vec::new();

    for (idx, word) in words.iter().enumerate() {
        let lower = word.to_lowercase();
        if idx > 0 && stop_words.contains(&lower.as_str()) {
            titled.push(lower);
        } else {
            let mut chars = word.chars();
            if let Some(first) = chars.next() {
                titled.push(format!("{}{}", first.to_uppercase(), chars.as_str().to_lowercase()));
            }
        }
    }

    titled.join(" ")
}

fn category_default_title(cat: Category) -> &'static str {
    match cat {
        Category::Combat => "Combat & Tactics",
        Category::Exploration => "World Exploration",
        Category::Choices => "Quests & Decisions",
        Category::Survival => "Health & Survival",
        Category::Crafting => "Crafting & Equipment",
        Category::Stealth => "Stealth & Infiltration",
        Category::Coop => "Cooperative Play",
        Category::Progression => "Character Progression",
        Category::Puzzle => "Puzzles & Environment",
        Category::General => "Core Gameplay",
    }
}

fn split_sentences(text: &str) -> Vec<&str> {
    let mut sentences: Vec<&str> = Vec::new();
    let mut start = 0;
    let chars: Vec<(usize, char)> = text.char_indices().collect();

    for i in 0..chars.len() {
        let (_, c) = chars[i];
        if (c == '.' || c == '!' || c == '?') && (i + 1 == chars.len() || chars[i + 1].1.is_whitespace()) {
            let end = chars[i].0 + c.len_utf8();
            let s = text[start..end].trim();
            if !s.is_empty() {
                sentences.push(s);
            }
            start = end;
        }
    }

    if sentences.is_empty() && !text.trim().is_empty() {
        sentences.push(text.trim());
    }

    sentences
}

fn clean_sentence(s: &str) -> String {
    let mut text = s.trim().to_string();

    for re in RE_PREFIXES.iter() {
        text = re.replace(&text, "").to_string();
    }

    text = RE_PLAYER_CAN.replace_all(&text, "Players can ").to_string();
    text = RE_PLAYER_MUST.replace_all(&text, "Players must ").to_string();
    text = RE_PLAYER_EARNS.replace_all(&text, "Players earn ").to_string();
    text = RE_PLAYER_MAINTAINS.replace_all(&text, "Maintains ").to_string();
    text = RE_GAME_FEATURES.replace_all(&text, "Features ").to_string();

    for re in RE_FILLERS.iter() {
        text = re.replace_all(&text, "").to_string();
    }

    // Capitalize first character
    let mut chars = text.chars();
    if let Some(first) = chars.next() {
        text = format!("{}{}", first.to_uppercase(), chars.as_str());
    }

    text.trim().to_string()
}

fn is_pure_boilerplate(s: &str) -> bool {
    let lower = s.to_lowercase();

    let is_exposition = lower.contains("developed by")
        || lower.contains("published by")
        || lower.contains("engine")
        || lower.contains("similar to those found in")
        || lower.contains("elements similar to")
        || lower.contains("inherited from")
        || lower.contains("leaving their corporate job")
        || lower.contains("town's revival")
        || lower.contains("received critical acclaim")
        || lower.contains("critical reception")
        || lower.contains("nominated for")
        || lower.contains("first single from")
        || lower.contains("video game soundtrack")
        || lower.contains("released for")
        || lower.contains("released on")
        || lower.contains("features elements commonly found in role-playing");

    if is_exposition {
        return true;
    }

    if (lower.starts_with("it is an ") || lower.starts_with("it is a ")) && lower.ends_with("game.") {
        return true;
    }

    if (lower.contains("is a ") || lower.contains("is an "))
        && lower.contains("game")
        && !lower.contains("where")
        && !lower.contains("in which")
        && !lower.contains("that")
        && !lower.contains("emphasize")
        && !lower.contains("features")
        && !lower.contains("customiz")
        && !lower.contains("combat")
        && !lower.contains("turn-based")
    {
        return true;
    }

    let gameplay_indicators = [
        "combat", "turn-based", "tactics", "tactical", "weapon", "weapons", "sword", "swords",
        "shooter", "shooting", "shoot", "gun", "guns", "melee", "attack", "damage", "enemies",
        "enemy", "cover", "reload", "health", "bleed", "bleed-out", "revive", "revived", "death",
        "permadeath", "permanent death", "injuries", "action point", "action points", "squad",
        "mercenaries", "classes", "class", "skills", "skill", "abilities", "ability", "astromech",
        "droids", "procedurally generated", "base of operations", "the den", "research", "upgrade",
        "upgrades", "cycles", "operations", "gather intel", "narrative decisions", "consequences",
        "sabotage", "dialogue", "dialogue tree", "dialogue trees", "choice", "choices",
        "reputation", "bonds", "synergy", "synergies", "superhero", "dispatch", "minigame",
        "hacking", "quick time", "farm", "farming", "seeds", "crops", "harvest", "livestock",
        "barns", "coops", "bundles", "community center", "lobby", "lobbies", "proximity chat",
        "voice", "voices", "inaudible", "volume", "chat", "puzzles", "puzzle", "stealth", "level",
        "leveling", "xp", "experience",
        "craft", "crafting", "mount", "steed", "fast travel", "open world", "explore",
        "exploration", "navigate", "quests", "quest", "needs", "sleep", "eat", "magic", "spells",
        "backstabs", "backstab", "attributes", "raise animals", "raising animals", "monsters",
        "mining", "fishing", "foraging", "marry", "marriage",
    ];

    let has_gp = gameplay_indicators.iter().any(|kw| contains_word(&lower, kw));
    !has_gp
}

fn category_to_icon(cat: Category) -> &'static str {
    match cat {
        Category::Combat => "combat",
        Category::Exploration => "explore",
        Category::Choices => "choices",
        Category::Survival => "survival",
        Category::Crafting => "crafting",
        Category::Stealth => "stealth",
        Category::Coop => "coop",
        Category::Progression => "progression",
        Category::Puzzle => "puzzle",
        Category::General => "combat",
    }
}

fn category_to_id(cat: Category) -> &'static str {
    match cat {
        Category::Combat => "combat",
        Category::Exploration => "explore",
        Category::Choices => "choices",
        Category::Survival => "survival",
        Category::Crafting => "crafting",
        Category::Stealth => "stealth",
        Category::Coop => "coop",
        Category::Progression => "progression",
        Category::Puzzle => "puzzle",
        Category::General => "core",
    }
}

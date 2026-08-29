use regex::Regex;
use std::sync::LazyLock;

static RE_REF_SELF: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?is)<ref[^>]*/>").unwrap());
static RE_REF_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?is)<ref(?:\s+[^>/]*|)>.*?</ref>").unwrap());
static RE_HTML_TAGS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?is)<[^>]+>").unwrap());
static RE_BOLD_ITALIC: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"'''+|''+").unwrap());
static RE_WIKILINK_PIPED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[\[(?:[^|\]]+\|)([^\]]+)\]\]").unwrap());
static RE_WIKILINK_SIMPLE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[\[([^\]]+)\]\]").unwrap());
static RE_BRACKET_CITATIONS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[(?:\d+|citation needed|note \d+|nb \d+)\]").unwrap());

pub struct CleanedWikitext {
    pub text: String,
    pub image_files: Vec<String>,
}

pub fn clean_wikitext(raw: &str) -> CleanedWikitext {
    // 1. Strip self-closing ref tags FIRST so <ref .../> is not treated as unclosed <ref>
    let s = RE_REF_SELF.replace_all(raw, "");
    let s = RE_REF_BLOCK.replace_all(&s, "");

    // 2. Strip File/Image embeds (handling nested [[...]] links in captions)
    let (s, image_files) = remove_file_embeds(&s);

    // 3. Strip templates {{...}} (handling simple nesting)
    let s = remove_templates(&s);

    // 4. Flatten wikilinks [[Target|Text]] -> Text and [[Text]] -> Text
    let s = RE_WIKILINK_PIPED.replace_all(&s, "$1");
    let s = RE_WIKILINK_SIMPLE.replace_all(&s, "$1");

    // 4.5. Strip inline bracket citations like [4] or [citation needed]
    let s = RE_BRACKET_CITATIONS.replace_all(&s, "");

    // 5. Strip formatting and html tags
    let s = RE_BOLD_ITALIC.replace_all(&s, "");
    let s = RE_HTML_TAGS.replace_all(&s, "");

    // 6. Decode entities
    let s = s
        .replace("&nbsp;", " ")
        .replace("&#160;", " ")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">");

    // 7. Normalize paragraphs and trim
    let mut cleaned_paragraphs: Vec<String> = Vec::new();
    let mut current_para: Vec<String> = Vec::new();

    for line in s.lines() {
        let trimmed = line.trim();
        // Ignore section headers like ==Gameplay== or ===Sub===
        if trimmed.starts_with("==") && trimmed.ends_with("==") {
            continue;
        }
        if trimmed.is_empty() {
            if !current_para.is_empty() {
                cleaned_paragraphs.push(current_para.join(" "));
                current_para.clear();
            }
        } else {
            current_para.push(trimmed.to_string());
        }
    }
    if !current_para.is_empty() {
        cleaned_paragraphs.push(current_para.join(" "));
    }

    let cleaned_text = cleaned_paragraphs.join("\n\n");

    CleanedWikitext {
        text: cleaned_text,
        image_files,
    }
}


fn remove_file_embeds(input: &str) -> (String, Vec<String>) {
    let mut result = String::with_capacity(input.len());
    let mut image_files = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if i + 1 < len && chars[i] == '[' && chars[i + 1] == '[' {
            let rest_str: String = chars[i + 2..std::cmp::min(i + 10, len)].iter().collect();
            let rest_lower = rest_str.to_lowercase();
            let is_file = rest_lower.starts_with("file:");
            let is_image = rest_lower.starts_with("image:");

            if is_file || is_image {
                let prefix_len = if is_file { 7 } else { 8 };
                let mut depth = 1;
                let mut embed_end = len;
                let mut file_name_end = None;
                let mut j = i + prefix_len;

                while j < len {
                    if file_name_end.is_none() && (chars[j] == '|' || (j + 1 < len && chars[j] == ']' && chars[j + 1] == ']')) {
                        file_name_end = Some(j);
                    }
                    if j + 1 < len && chars[j] == '[' && chars[j + 1] == '[' {
                        depth += 1;
                        j += 2;
                    } else if j + 1 < len && chars[j] == ']' && chars[j + 1] == ']' {
                        depth -= 1;
                        if depth == 0 {
                            embed_end = j + 2;
                            break;
                        }
                        j += 2;
                    } else {
                        j += 1;
                    }
                }

                let name_end = file_name_end.unwrap_or(j);
                if name_end >= i + prefix_len {
                    let raw_name: String = chars[i + prefix_len..name_end].iter().collect();
                    let clean_name = raw_name.trim().to_string();
                    if !clean_name.is_empty() && !image_files.contains(&clean_name) {
                        image_files.push(clean_name);
                    }
                }

                i = embed_end;
                continue;
            }
        }

        result.push(chars[i]);
        i += 1;
    }

    (result, image_files)
}

fn remove_templates(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut depth = 0;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '{' && chars.peek() == Some(&'{') {
            chars.next();
            depth += 1;
        } else if c == '}' && chars.peek() == Some(&'}') {
            chars.next();
            if depth > 0 {
                depth -= 1;
            }
        } else if depth == 0 {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_wikitext_refs_and_links() {
        let raw = r#"
== Gameplay ==
[[File:Gears 3 gameplay.jpg|thumb|Combat]]
''Gears of War 3'' is a [[third-person shooter]] that emphasizes the use of cover.<ref>{{Cite web|url=test.com}}</ref>
Players can carry four weapons.<ref name="ign" />
"#;
        let cleaned = clean_wikitext(raw);
        assert_eq!(cleaned.image_files, vec!["Gears 3 gameplay.jpg"]);
        assert!(cleaned.text.contains("Gears of War 3 is a third-person shooter that emphasizes the use of cover."));
        assert!(cleaned.text.contains("Players can carry four weapons."));
        assert!(!cleaned.text.contains("<ref"));
        assert!(!cleaned.text.contains("[[File"));
    }
}

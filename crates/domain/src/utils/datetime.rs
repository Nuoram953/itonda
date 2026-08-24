pub fn parse_timestamp(s: &str) -> Option<i64> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Ok(ts) = trimmed.parse::<i64>() {
        return Some(ts);
    }
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(trimmed) {
        return Some(dt.timestamp());
    }
    if let Ok(dt) = chrono::DateTime::parse_from_rfc2822(trimmed) {
        return Some(dt.timestamp());
    }
    let without_utc = trimmed.strip_suffix(" UTC").unwrap_or(trimmed);
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(without_utc, "%Y-%m-%d %H:%M:%S") {
        return Some(dt.and_utc().timestamp());
    }
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(without_utc, "%Y-%m-%d %H:%M:%S%.f") {
        return Some(dt.and_utc().timestamp());
    }
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(without_utc, "%Y-%m-%dT%H:%M:%S") {
        return Some(dt.and_utc().timestamp());
    }
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(without_utc, "%Y-%m-%dT%H:%M:%S%.f") {
        return Some(dt.and_utc().timestamp());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numeric_timestamp() {
        assert_eq!(parse_timestamp("1724490000"), Some(1724490000));
        assert_eq!(parse_timestamp(" 1724490000 "), Some(1724490000));
    }

    #[test]
    fn test_parse_rfc3339() {
        assert_eq!(parse_timestamp("2026-08-23T11:30:00Z"), Some(1787484600));
        assert_eq!(
            parse_timestamp("2026-08-23T13:30:00+02:00"),
            Some(1787484600)
        );
    }

    #[test]
    fn test_parse_utc_suffix() {
        assert_eq!(parse_timestamp("2026-08-23 11:30:00 UTC"), Some(1787484600));
        assert_eq!(
            parse_timestamp("2026-08-23 11:30:00.123 UTC"),
            Some(1787484600)
        );
    }

    #[test]
    fn test_parse_naive_datetime() {
        assert_eq!(parse_timestamp("2026-08-23 11:30:00"), Some(1787484600));
        assert_eq!(parse_timestamp("2026-08-23T11:30:00"), Some(1787484600));
    }

    #[test]
    fn test_parse_invalid_and_empty() {
        assert_eq!(parse_timestamp(""), None);
        assert_eq!(parse_timestamp("   "), None);
        assert_eq!(parse_timestamp("invalid-date-string"), None);
    }
}

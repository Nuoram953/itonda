use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum VdfParseError {
    #[error("Unexpected end of input")]
    UnexpectedEof,
    #[error("Expected token '{expected}', found '{found}'")]
    UnexpectedToken { expected: String, found: String },
    #[error("Unterminated string literal")]
    UnterminatedString,
    #[error("Missing required field '{0}'")]
    MissingField(String),
}

#[derive(Debug, Error)]
pub enum ScannerError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("vdf parse error: {0}")]
    VdfParse(#[from] VdfParseError),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("scanner is unavailable: {0}")]
    Unavailable(String),

    #[error("scanner path not found: {0}")]
    PathNotFound(PathBuf),

    #[error("parse error: {0}")]
    Parse(String),

    #[error("scanner error: {0}")]
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let scanner_err: ScannerError = io_err.into();
        assert!(matches!(scanner_err, ScannerError::Io(_)));
        assert_eq!(scanner_err.to_string(), "io error: file not found");
    }

    #[test]
    fn test_vdf_error_conversion() {
        let vdf_err = VdfParseError::UnexpectedEof;
        let scanner_err: ScannerError = vdf_err.into();
        assert!(matches!(scanner_err, ScannerError::VdfParse(_)));
        assert_eq!(
            scanner_err.to_string(),
            "vdf parse error: Unexpected end of input"
        );
    }

    #[test]
    fn test_custom_variants() {
        let err = ScannerError::Unavailable("Steam".into());
        assert_eq!(err.to_string(), "scanner is unavailable: Steam");

        let err = ScannerError::PathNotFound(PathBuf::from("/games/library"));
        assert_eq!(err.to_string(), "scanner path not found: /games/library");

        let err = ScannerError::Parse("invalid manifest".into());
        assert_eq!(err.to_string(), "parse error: invalid manifest");

        let err = ScannerError::Other("unknown error".into());
        assert_eq!(err.to_string(), "scanner error: unknown error");
    }
}

use thiserror::Error;

/// Search error types
#[derive(Error, Debug)]
pub enum SearchError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Rate limited by search engine")]
    RateLimited,

    #[error("Search engine blocked the request")]
    Blocked,

    #[error("Invalid response from search engine: {0}")]
    InvalidResponse(String),

    #[error("Request timeout")]
    Timeout,

    #[error("Other error: {0}")]
    Other(String),

    #[error("No search results found")]
    NoResults,

    #[error("Unknown error occurred")]
    Unknown,
}

/// Configuration error types
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to find home directory")]
    NoHomeDir,
    #[error("Failed to find config directory")]
    NoConfigDir,
    #[error("Failed to create config directory: {0}")]
    CreateDir(#[from] std::io::Error),
    #[error("Failed to parse config file: {0}")]
    ParseError(#[from] toml::de::Error),
    #[error("Invalid configuration value: {0}")]
    ValidationError(String),
}

use thiserror::Error;

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

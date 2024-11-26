use std::time::Duration;

// Number of search results per page
pub const ITEMS_PER_PAGE: usize = 10;
// Event polling timeout duration
pub const POLL_TIMEOUT: Duration = Duration::from_millis(100);
// Search rate limit duration
pub const RATE_LIMIT_DURATION: Duration = Duration::from_secs(1);

/// Search result structure
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub title: String,       // Search result title
    pub url: String,         // Search result URL
    pub description: String, // Search result description
}

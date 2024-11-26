use rand::{seq::SliceRandom, thread_rng, Rng};
use std::time::Duration;
use tokio::time::sleep;
use anyhow::Result;
use tokio::time::timeout;
use once_cell::sync::Lazy;
use moka::future::Cache;

use crate::{SearchError, SearchResult, CONFIG};

/// Search Engine Enum
#[derive(Debug, Clone, Copy)]
pub enum SearchEngine {
    Google(Google),
    Bing(Bing),
    DuckDuckGo(DuckDuckGo),
}

impl SearchEngine {
    pub fn favor(engine_name: &str) -> Self {
        match engine_name.to_lowercase().as_str() {
            "google" => SearchEngine::Google(Google),
            "bing" => SearchEngine::Bing(Bing),
            "duckduckgo" => SearchEngine::DuckDuckGo(DuckDuckGo),
            _ => SearchEngine::Google(Google),
        }
    }

    /// Get the next search engine in the sequence
    pub fn next(&self) -> Self {
        match self {
            SearchEngine::Google(_) => SearchEngine::Bing(Bing),
            SearchEngine::Bing(_) => SearchEngine::DuckDuckGo(DuckDuckGo),
            SearchEngine::DuckDuckGo(_) => SearchEngine::Google(Google),
        }
    }

    /// Get the name of the search engine as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            SearchEngine::Google(google) => google.name(),
            SearchEngine::Bing(bing) => bing.name(),
            SearchEngine::DuckDuckGo(duck_duck_go) => duck_duck_go.name(),
        }
    }

    /// Perform a search using the current search engine
    pub async fn search(&self, query: &str, start: u16) -> Result<Vec<SearchResult>> {
        match self {
            SearchEngine::Google(google) => google.search(query, start).await,
            SearchEngine::Bing(bing) => bing.search(query, start).await,
            SearchEngine::DuckDuckGo(duck_duck_go) => duck_duck_go.search(query, start).await,
        }
    }
}

/// Trait for search engines
pub trait Engine {
    /// Get the name of the search engine
    fn name(&self) -> &'static str;
    /// Build the URL for a search query
    fn build_url(&self, query: &str, start: u16) -> String;
    /// Perform a search
    fn search(&self, query: &str, start: u16) -> impl std::future::Future<Output = Result<Vec<SearchResult>>>;
}

/// Google search engine
#[derive(Debug, Clone, Copy)]
pub struct Google;

impl Engine for Google {
    /// Get the name of the search engine
    fn name(&self) -> &'static str {
        "Google"
    }

    /// Build the URL for a search query
    fn build_url(&self, query: &str, start: u16) -> String {
        format!(
            "https://www.google.com/search?q={}&num=10&start={}",
            urlencoding::encode(query),
            start
        )
    }

    /// Perform a search
    async fn search(&self, query: &str, start: u16) -> Result<Vec<SearchResult>> {
        let url = self.build_url(query, start);
        let text = fetch_response_text(&url).await?;
        let document = scraper::Html::parse_document(&text);

        let container_selector = scraper::Selector::parse("div.MjjYud").unwrap();
        let title_selector = scraper::Selector::parse("h3.LC20lb").unwrap();
        let link_selector = scraper::Selector::parse("a[jsname='UWckNb']").unwrap();
        let description_selector = scraper::Selector::parse("div.VwiC3b").unwrap();

        let mut results = Vec::new();
        let mut seen_urls = std::collections::HashSet::new();

        for container in document.select(&container_selector) {
            if let Some(link_element) = container.select(&link_selector).next() {
                if let Some(href) = link_element.value().attr("href") {
                    if href.starts_with("http") && !seen_urls.contains(href) {
                        let title = container
                            .select(&title_selector)
                            .next()
                            .map(|el| el.text().collect::<String>().trim().to_string())
                            .unwrap_or_else(|| "No title".to_string());

                        let description = container
                            .select(&description_selector)
                            .next()
                            .map(|el| el.text().collect::<String>().trim().to_string())
                            .unwrap_or_else(|| "No description".to_string());

                        seen_urls.insert(href.to_string());
                        results.push(SearchResult {
                            title,
                            url: href.to_string(),
                            description,
                        });
                    }
                }
            }
         }

        if results.is_empty() {
            return Err(anyhow::anyhow!(SearchError::NoResults));
        }

        Ok(results)
    }
}

/// Bing search engine
#[derive(Debug, Clone, Copy)]
pub struct Bing;

impl Engine for Bing {
    /// Get the name of the search engine
    fn name(&self) -> &'static str {
        "Bing"
    }

    /// Build the URL for a search query
    fn build_url(&self, query: &str, start: u16) -> String {
        format!(
            "https://www.bing.com/search?q={}&count=10&first={}",
            urlencoding::encode(query),
            start
        )
    }

    /// Perform a search
    async fn search(&self, query: &str, start: u16) -> Result<Vec<SearchResult>> {
        let url = self.build_url(query, start);
        let text = fetch_response_text(&url).await?;
        let document = scraper::Html::parse_document(&text);

        let container_selector = scraper::Selector::parse("li.b_algo").unwrap();
        let title_selector = scraper::Selector::parse("h2").unwrap();
        let link_selector = scraper::Selector::parse("h2 a").unwrap();
        let description_selector = scraper::Selector::parse("div.b_caption p").unwrap();

        let mut results = Vec::new();
        let mut seen_urls = std::collections::HashSet::new();

        for container in document.select(&container_selector) {
            if let Some(link_element) = container.select(&link_selector).next() {
                if let Some(href) = link_element.value().attr("href") {
                    if href.starts_with("http") && !seen_urls.contains(href) {
                        let title = container
                            .select(&title_selector)
                            .next()
                            .map(|el| el.text().collect::<String>().trim().to_string())
                            .unwrap_or_else(|| "No title".to_string());

                        let description = container
                            .select(&description_selector)
                            .next()
                            .map(|el| el.text().collect::<String>().trim().to_string())
                            .unwrap_or_else(|| "No description".to_string());

                        seen_urls.insert(href.to_string());
                        results.push(SearchResult {
                            title,
                            url: href.to_string(),
                            description,
                        });
                    }
                }
            }
        }

        if results.is_empty() {
            return Err(anyhow::anyhow!(SearchError::NoResults));
        }

        Ok(results)
    }
}

/// DuckDuckGo search engine
#[derive(Debug, Clone, Copy)]
pub struct DuckDuckGo;

impl Engine for DuckDuckGo {
    /// Get the name of the search engine
    fn name(&self) -> &'static str {
        "DuckDuckGo"
    }

    /// Build the URL for a search query
    fn build_url(&self, query: &str, start: u16) -> String {
        format!(
            "https://html.duckduckgo.com/html/?q={}&s={}",
            urlencoding::encode(query),
            start
        )
    }

    /// Perform a search
    async fn search(&self, query: &str, start: u16) -> Result<Vec<SearchResult>> {
        let url = self.build_url(query, start);
        let text = fetch_response_text(&url).await?;
        let document = scraper::Html::parse_document(&text);

        let result_selector = scraper::Selector::parse(".result").unwrap();
        let title_selector = scraper::Selector::parse(".result__title a").unwrap();
        let snippet_selector = scraper::Selector::parse(".result__snippet").unwrap();

        let mut results = Vec::new();
        let mut seen_urls = std::collections::HashSet::new();

        for result in document.select(&result_selector) {
            if let (Some(title_elem), Some(snippet_elem)) = (
                result.select(&title_selector).next(),
                result.select(&snippet_selector).next(),
            ) {
                if let Some(url) = title_elem.value().attr("href") {
                    let title = title_elem.text().collect::<String>();
                    let description = snippet_elem.text().collect::<String>();
                    
                    let clean_url = if url.starts_with("http") {
                        url.to_string()
                    } else if url.starts_with("//") {
                        format!("https:{}", url)
                    } else if url.starts_with('/') {
                        if let Some(actual_url) = url.split("uddg=").nth(1) {
                            if let Ok(decoded) = urlencoding::decode(actual_url) {
                                decoded.into_owned()
                            } else {
                                continue;
                            }
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    };

                    // Skip if we've already seen this URL
                    if !seen_urls.insert(clean_url.clone()) {
                        continue;
                    }

                    results.push(SearchResult {
                        title: title.trim().to_string(),
                        url: clean_url,
                        description: description.trim().to_string(),
                    });
                }
            }
        }

        if results.is_empty() {
            return Err(anyhow::anyhow!(SearchError::NoResults));
        }

        Ok(results)
    }
}  

static RESPONSE_CACHE: Lazy<Cache<String, String>> = Lazy::new(|| {
    Cache::builder()
        .max_capacity(CONFIG.cache.max_capacity)
        .time_to_live(Duration::from_secs(CONFIG.cache.time_to_live)) // 10 minutes expiration
        .build()
});

/// Fetch the response text from a URL
async fn fetch_response_text(url: &str) -> Result<String> {
    // First, try to get the response from the cache
    if let Some(cached_response) = RESPONSE_CACHE.get(url).await {
        return Ok(cached_response);
    }

    let mut rng = thread_rng();

    // Randomly select a User-Agent
    let user_agent = CONFIG.search.user_agents.choose(&mut rng).unwrap_or(&CONFIG.search.user_agents[0]);

    // Add a random delay
    let jitter = rng.gen_range(0..CONFIG.search.max_jitter);
    sleep(Duration::from_millis(CONFIG.search.base_delay + jitter)).await;

    let client = reqwest::Client::builder()
        .user_agent(user_agent.to_string())
        .cookie_store(true)
        .timeout(Duration::from_secs(CONFIG.search.request_timeout))
        .build()
        .map_err(|e| SearchError::Other(format!("Failed to build client: {}", e)))?;

    let mut last_error = None;

    // Retry mechanism
    for retry in 0..CONFIG.search.max_retries {
        if retry > 0 {
            // If it's a retry, increase the delay time
            sleep(Duration::from_millis(
                (CONFIG.search.base_delay * (retry as u64)) + jitter,
            ))
            .await;
        }

        // Use the timeout package to wrap the entire request process
        match timeout(
            Duration::from_secs(CONFIG.search.request_timeout),
            async {
                match client
                    .get(url)
                    .header(
                        "Accept",
                        "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
                    )
                    .header("Accept-Language", "en-US,en;q=0.9")
                    .header("Accept-Encoding", "identity")
                    .send()
                    .await
                {
                    Ok(response) => {
                        let status = response.status();
                        
                        match status.as_u16() {
                            200 => {
                                // Use the timeout package to wrap the response body reading
                                match timeout(
                                    Duration::from_secs(CONFIG.search.response_timeout),
                                    response.text()
                                ).await {
                                    Ok(Ok(text)) => {
                                        if text.contains("detected unusual traffic") 
                                           || text.contains("CAPTCHA") 
                                           || text.contains("blocked") {
                                            Err(SearchError::Blocked)
                                        } else {
                                            Ok(text)
                                        }
                                    }
                                    Ok(Err(e)) => {
                                        Err(SearchError::InvalidResponse(e.to_string()))
                                    }
                                    Err(_) => {
                                        Err(SearchError::Timeout)
                                    }
                                }
                            }
                            429 => Err(SearchError::RateLimited),
                            403 => Err(SearchError::Blocked),
                            408 | 504 => Err(SearchError::Timeout),
                            _ => Err(SearchError::NetworkError(
                                format!("HTTP error: {}", status)
                            )),
                        }
                    }
                    Err(e) => {
                        if e.is_timeout() {
                            Err(SearchError::Timeout)
                        } else if e.is_connect() {
                            Err(SearchError::NetworkError(
                                "Connection failed".to_string()
                            ))
                        } else {
                            Err(SearchError::NetworkError(e.to_string()))
                        }
                    }
                }
            }
        ).await {
            Ok(Ok(text)) => {
                // Store the response in the cache
                RESPONSE_CACHE.insert(url.to_string(), text.clone()).await;
                return Ok(text);
            }
            Ok(Err(e)) => last_error = Some(e),
            Err(_) => last_error = Some(SearchError::Timeout),
        }
    }

    Err(anyhow::anyhow!(last_error.unwrap_or(SearchError::Unknown)))
}

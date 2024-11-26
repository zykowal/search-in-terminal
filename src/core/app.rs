use anyhow::Result;
use std::time::Instant;

use ratatui::widgets::ListState;

use crate::{
    search::{
        engine::SearchEngine,
        models::{ITEMS_PER_PAGE, RATE_LIMIT_DURATION},
    },
    SearchResult, CONFIG,
};

/// Application state structure
#[derive(Debug)]
pub struct App {
    /// User search query input
    pub input: String,
    
    /// List of search results
    pub search_results: Vec<SearchResult>,
    
    /// Currently selected result index
    pub selected_index: usize,
    
    /// Whether in input mode
    pub input_mode: bool,
    
    /// Error message
    pub error_message: Option<String>,
    
    /// Warning message
    pub warning_message: Option<String>,
    
    /// Current scroll position in the results list
    pub scroll_offset: usize,
    
    /// Current page number (0-based)
    pub page: usize,
    
    /// Total number of available pages
    pub total_pages: usize,
    
    /// Timestamp of the last search operation
    pub last_search: Option<Instant>,
    
    /// Flag indicating whether a search is in progress
    pub is_loading: bool,
    
    /// Current search engine being used
    pub search_engine: SearchEngine,
    
    /// State of the results list selection
    pub list_state: ListState,
    
    /// Starting index for pagination
    pub start: u16,
}

impl App {
    /// Creates a new application instance with default settings
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            input: String::new(),
            search_results: Vec::new(),
            selected_index: 0,
            input_mode: true,
            error_message: None,
            warning_message: None,
            scroll_offset: 0,
            page: 0,
            total_pages: 0,
            last_search: None,
            is_loading: false,
            search_engine: SearchEngine::favor(&CONFIG.engine.favor),
            list_state,
            start: 0,
        }
    }

    /// Sets the total number of pages for search results
    pub fn total_pages(&mut self) {
        if self.search_results.is_empty() {
            self.total_pages = 1;
        } else {
            self.total_pages = (self.search_results.len() + ITEMS_PER_PAGE - 1) / ITEMS_PER_PAGE;
        }
    }

    /// Gets the range of results for the current page
    pub fn current_page_range(&self) -> (usize, usize) {
        if self.search_results.is_empty() {
            return (0, 0);
        }

        let start_index = self.page * ITEMS_PER_PAGE;
        let end_index = if self.page == self.total_pages - 1 {
            self.search_results.len()
        } else {
            start_index + ITEMS_PER_PAGE
        };
        (start_index, end_index)
    }

    /// Switches to the next or previous page
    pub async fn change_page(&mut self, direction: i32) -> Result<()> {
        if self.search_results.is_empty() {
            return Ok(());
        }

        if let Some(new_page) = (self.page as i32).checked_add(direction) {
            if new_page >= 0 && new_page < self.total_pages as i32 {
                self.page = new_page as usize;
                let (start_index, _) = self.current_page_range();
                self.selected_index = start_index;
                self.scroll_offset = 0;
                self.list_state.select(Some(0));
            } else if new_page >= 0 && direction > 0 {
                // If attempting to go to the next page but already on the last page, try to get more results
                self.next_search().await?;
                if self.page < self.total_pages - 1 {
                    self.page += 1;
                    let (start_index, _) = self.current_page_range();
                    self.selected_index = start_index;
                    self.scroll_offset = 0;
                    self.list_state.select(Some(0));
                }
            }
        }

        Ok(())
    }

    /// Clears the input field
    pub fn clear_input(&mut self) {
        self.input.clear();
    }

    /// Clears the search results
    pub fn clear_results(&mut self) {
        self.search_results.clear();
        self.selected_index = 0;
        self.page = 0;
        self.total_pages = 0;
        self.scroll_offset = 0;
        self.list_state.select(Some(0));
        self.error_message = None;
        self.warning_message = None;
        self.input.clear();
        self.input_mode = true;
        self.start = 0;
    }

    pub async fn next_search(&mut self) -> Result<()> {
        if self.input.is_empty() {
            return Ok(());
        }

        self.error_message = None;
        self.warning_message = None;

        // Update the start position
        self.start = self.start.saturating_add(10);

        // Perform the search using the selected search engine
        let results = match self.search_engine.search(&self.input, self.start).await {
            Ok(results) => Ok(results),
            Err(e) => {
                self.error_message = Some(format!("Next search failed: {}", e));
                Err(e)
            }
        };

        match results {
            Ok(mut results) => {
                // Check for duplicate results
                let existing_urls: std::collections::HashSet<_> =
                    self.search_results.iter().map(|r| &r.url).collect();

                // Only add non-duplicate results
                results.retain(|result| !existing_urls.contains(&result.url));

                if !results.is_empty() {
                    self.search_results.extend(results);
                    self.total_pages();
                } else {
                    self.warning_message = Some("No more results found".to_string());
                }
            }
            Err(e) => {
                self.error_message = Some(format!("Next search failed: {}", e));
            }
        }

        Ok(())
    }

    /// Performs a search operation
    pub async fn perform_search(&mut self) -> Result<()> {
        if self.input.is_empty() {
            return Ok(());
        }

        // Check the search rate limit
        if let Some(last_search) = self.last_search {
            if last_search.elapsed() < RATE_LIMIT_DURATION {
                self.warning_message =
                    Some("Please wait a moment before searching again".to_string());
                return Ok(());
            }
        }

        self.is_loading = true;
        self.error_message = None;
        self.warning_message = None;
        self.start = 0; // Reset the start position

        // Perform the search using the selected search engine
        let results = match self.search_engine.search(&self.input, self.start).await {
            Ok(results) => Ok(results),
            Err(e) => {
                self.error_message = Some(format!("Search failed: {}", e));
                Err(e)
            }
        };

        match results {
            Ok(results) => {
                self.search_results = results;
                self.selected_index = 0;
                self.page = 0;
                self.scroll_offset = 0;
                self.list_state.select(Some(0));
                self.total_pages();
            }
            Err(e) => {
                self.error_message = Some(format!("Search failed: {}", e));
                self.clear_results();
            }
        }

        self.is_loading = false;
        self.input_mode = false;
        self.last_search = Some(Instant::now());
        Ok(())
    }

    /// Opens the selected URL
    pub fn open_selected_url(&mut self) -> Result<()> {
        let (start_index, _) = self.current_page_range();
        if let Some(selected) = self.list_state.selected() {
            if let Some(result) = self.search_results.get(start_index + selected) {
                open::that(&result.url)?;
            }
        }
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

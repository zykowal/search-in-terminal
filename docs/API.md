# API Documentation

## Core Components

### SearchEngine

The `SearchEngine` enum represents different search engines supported by the application.

```rust
pub enum SearchEngine {
    Google(Google),
    Bing(Bing),
    DuckDuckGo(DuckDuckGo),
}
```

#### Methods

- `next(&self) -> Self`: Switch to the next search engine in rotation
- `as_str(&self) -> &'static str`: Get the name of the current search engine
- `search(&self, query: &str, start: u16) -> Result<Vec<SearchResult>>`: Perform a search operation

### App

The `App` struct manages the application state and core functionality.

```rust
pub struct App {
    pub input: String,                     // User search query input
    pub search_results: Vec<SearchResult>, // List of search results
    pub selected_index: usize,             // Currently selected result index
    pub input_mode: bool,                  // Whether in input mode
    // ... other fields
}
```

#### Methods

- `new() -> Self`: Create a new application instance
- `perform_search(&mut self) -> Result<()>`: Execute a search operation
- `next_search(&mut self) -> Result<()>`: Load more search results
- `change_page(&mut self, direction: i32) -> Result<()>`: Navigate between pages
- `open_selected_url(&mut self) -> Result<()>`: Open the selected URL in browser

### Configuration

The `Config` struct handles application configuration.

```rust
pub struct Config {
    pub search_config: SearchConfig,
    pub cache_config: CacheConfig,
}
```

Configuration is loaded from:
- Linux/macOS: `~/.config/st/config.toml`
- Windows: `%APPDATA%\st\config.toml`

## Error Handling

The application uses the `thiserror` crate for error handling. Main error types:

```rust
pub enum SearchError {
    NetworkError(String),
    ParseError(String),
    NoResults,
    RateLimit,
    Other(String),
}
```

## Examples

### Basic Search Operation

```rust
use search_in_terminal::{App, SearchEngine};

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new();
    
    // Set search query
    app.input = "rust programming".to_string();
    
    // Perform search
    app.perform_search().await?;
    
    // Open first result in browser
    app.open_selected_url()?;
    
    Ok(())
}
```

### Custom Configuration

```rust
use search_in_terminal::Config;

let config = Config {
    search_config: SearchConfig {
        max_retries: 3,
        base_delay: 1000,
        max_jitter: 500,
        request_timeout: 10,
        response_timeout: 10,
        user_agents: vec![
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
        ],
    },
    cache_config: CacheConfig {
        max_capacity: 100,
        time_to_live: 600,
    },
};
```

### Switching Search Engines

```rust
use search_in_terminal::SearchEngine;

let mut engine = SearchEngine::Google(Google);
// Switch to next engine (Bing)
engine = engine.next();
// Switch to next engine (DuckDuckGo)
engine = engine.next();
```

## Event Handling

The application uses `crossterm` for terminal events:

```rust
use crossterm::event::{Event, KeyCode};

match event::read()? {
    Event::Key(key) => {
        match key.code {
            KeyCode::Enter => perform_search(),
            KeyCode::Tab => switch_engine(),
            KeyCode::Char('q') => quit(),
            // ... handle other keys
        }
    }
}
```

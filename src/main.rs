// Terminal Search Application
// A terminal-based search tool supporting Google, Bing, and DuckDuckGo
// with real-time search results display and interactive navigation

use anyhow::Result;
use search_in_terminal::run;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}

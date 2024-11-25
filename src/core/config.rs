use std::fs;

use once_cell::sync::Lazy;
use serde::Deserialize;

pub static CONFIG: Lazy<Config> = Lazy::new(Config::new);

#[derive(Debug, Deserialize)]
pub struct CacheConfig {
    pub max_capacity: u64,
    pub time_to_live: u64,
}

#[derive(Debug, Deserialize)]
pub struct SearchConfig {
    pub user_agents: Vec<String>,
    pub max_retries: u32,
    pub base_delay: u64,
    pub max_jitter: u64,
    pub request_timeout: u64,
    pub response_timeout: u64,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub search_config: SearchConfig,
    pub cache_config: CacheConfig,
}

impl Config {
    fn new() -> Self {
        if let Ok(content) = fs::read_to_string("config.toml") {
            toml::from_str(&content).unwrap_or(Config::default())
        } else {
            Config::default()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            search_config: SearchConfig {
                user_agents : vec![
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36".to_string(),
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36".to_string(),
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:122.0) Gecko/20100101 Firefox/122.0".to_string(),
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2.1 Safari/605.1.15".to_string(),
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36".to_string(),
                ],
                max_retries : 3,
                base_delay : 1000,
                max_jitter : 1000,
                request_timeout : 10,
                response_timeout : 10,
            },
            cache_config: CacheConfig {
                max_capacity: 100,
                time_to_live: 600,
            },
        }
    }
}

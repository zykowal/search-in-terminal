use anyhow::Result;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::error::types::ConfigError;

/// Global configuration instance, lazily initialized when first accessed
pub static CONFIG: Lazy<Config> =
    Lazy::new(|| Config::new().expect("Failed to load configuration"));

/// Cache configuration settings
#[derive(Debug, Deserialize)]
pub struct CacheConfig {
    /// Maximum number of items that can be stored in the cache
    #[serde(default = "default_max_capacity")]
    pub max_capacity: u64,

    /// Time-to-live for cached items in seconds
    #[serde(default = "default_time_to_live")]
    pub time_to_live: u64,
}

impl CacheConfig {
    fn validate(&self) -> Result<()> {
        if self.time_to_live == 0 {
            return Err(anyhow::anyhow!(ConfigError::ValidationError(
                "Cache time-to-live must be greater than 0".to_string(),
            )));
        }
        Ok(())
    }
}

/// Search engine configuration settings
#[derive(Debug, Deserialize)]
pub struct EngineConfig {
    /// Preferred search engine (google, bing, duckduckgo)
    #[serde(default = "default_favor")]
    pub favor: String,
}

impl EngineConfig {
    fn validate(&self) -> Result<()> {
        match self.favor.as_str() {
            "google" | "bing" | "duckduckgo" => Ok(()),
            _ => Err(anyhow::anyhow!(ConfigError::ValidationError(
                "Invalid search engine specified".to_string(),
            ))),
        }
    }
}

/// Search behavior configuration settings
#[derive(Debug, Deserialize)]
pub struct SearchConfig {
    /// List of user agents to rotate through for requests
    #[serde(default = "default_user_agents")]
    pub user_agents: Vec<String>,

    /// Maximum number of retry attempts for failed requests
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,

    /// Base delay between requests in milliseconds
    #[serde(default = "default_base_delay")]
    pub base_delay: u64,

    /// Maximum random jitter added to delay in milliseconds
    #[serde(default = "default_max_jitter")]
    pub max_jitter: u64,

    /// Request timeout in seconds
    #[serde(default = "default_request_timeout")]
    pub request_timeout: u64,

    /// Response timeout in seconds
    #[serde(default = "default_response_timeout")]
    pub response_timeout: u64,
}

impl SearchConfig {
    fn validate(&self) -> Result<()> {
        if self.max_retries > 10 {
            return Err(anyhow::anyhow!(ConfigError::ValidationError(
                "Max retries should not exceed 10".to_string(),
            )));
        }
        if self.request_timeout == 0 || self.response_timeout == 0 {
            return Err(anyhow::anyhow!(ConfigError::ValidationError(
                "Timeouts must be greater than 0".to_string(),
            )));
        }
        Ok(())
    }
}

/// Main configuration structure containing all settings
#[derive(Debug, Deserialize, Default)]
pub struct Config {
    /// Search-related settings
    #[serde(default)]
    pub search: SearchConfig,

    /// Cache-related settings
    #[serde(default)]
    pub cache: CacheConfig,

    /// Search engine preferences
    #[serde(default)]
    pub engine: EngineConfig,
}

impl Config {
    /// Creates a new Config instance by reading from the configuration file
    ///
    /// The configuration file is located at:
    /// - Windows: %APPDATA%\st\config.toml or %USERPROFILE%\AppData\Roaming\st\config.toml
    /// - Other: ~/.config/st/config.toml
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Home directory cannot be found
    /// - Config directory cannot be created
    /// - Config file cannot be parsed
    /// - Configuration values are invalid
    pub fn new() -> Result<Self> {
        let config_path = Self::config_path()?;
        Self::ensure_config_dir(&config_path)?;

        let config = if let Ok(content) = fs::read_to_string(&config_path) {
            toml::from_str(&content).unwrap_or(Config::default())
        } else {
            Config::default()
        };

        config.validate()?;
        Ok(config)
    }

    /// Returns the path to the configuration file
    fn config_path() -> Result<PathBuf> {
        let config_dir = if cfg!(target_os = "windows") {
            dirs::config_dir().unwrap_or_else(|| {
                dirs::home_dir()
                    .expect("Home directory not found")
                    .join("AppData")
                    .join("Roaming")
            })
        } else {
            dirs::home_dir()
                .ok_or(ConfigError::NoHomeDir)?
                .join(".config")
        };

        Ok(config_dir.join("st").join("config.toml"))
    }

    /// Ensures the configuration directory exists
    fn ensure_config_dir(config_path: &Path) -> Result<()> {
        if let Some(parent) = config_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        Ok(())
    }

    /// Validates all configuration values
    fn validate(&self) -> Result<()> {
        self.search.validate()?;
        self.cache.validate()?;
        self.engine.validate()?;
        Ok(())
    }
}

/// Default list of user agents for request rotation
fn default_user_agents() -> Vec<String> {
    vec![
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36".to_string(),
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36".to_string(),
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:122.0) Gecko/20100101 Firefox/122.0".to_string(),
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2.1 Safari/605.1.15".to_string(),
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36".to_string(),
    ]
}

// Default values for configuration settings
fn default_max_retries() -> u32 {
    3
}
fn default_base_delay() -> u64 {
    1000
}
fn default_max_jitter() -> u64 {
    1000
}
fn default_request_timeout() -> u64 {
    10
}
fn default_response_timeout() -> u64 {
    10
}
fn default_max_capacity() -> u64 {
    100
}
fn default_time_to_live() -> u64 {
    600
}
fn default_favor() -> String {
    "google".to_string()
}

/// Default implementation for SearchConfig
impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            user_agents: default_user_agents(),
            max_retries: default_max_retries(),
            base_delay: default_base_delay(),
            max_jitter: default_max_jitter(),
            request_timeout: default_request_timeout(),
            response_timeout: default_response_timeout(),
        }
    }
}

/// Default implementation for CacheConfig
impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_capacity: default_max_capacity(),
            time_to_live: default_time_to_live(),
        }
    }
}

/// Default implementation for EngineConfig
impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            favor: default_favor(),
        }
    }
}

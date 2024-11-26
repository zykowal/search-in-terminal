# Configuration Guide

## Configuration File Location

The configuration file should be named `config.toml` and placed in:
- Linux/macOS: `~/.config/st/config.toml`
- Windows: `%APPDATA%\st\config.toml` or `%USERPROFILE%\AppData\Roaming\st\config.toml`

The configuration directory will be automatically created if it doesn't exist.

## Configuration Format

The configuration file uses TOML format. All configurations are optional - if not specified, default values will be used. Here's a complete example with all available options:

```toml
[search]
# List of user agents to rotate through
user_agents = [
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36"
]
# Maximum number of retry attempts for failed requests
max_retries = 3
# Base delay between requests in milliseconds
base_delay = 1000
# Maximum random jitter added to delay in milliseconds
max_jitter = 500
# Request timeout in seconds
request_timeout = 10
# Response timeout in seconds
response_timeout = 10

[cache]
# Maximum number of cached items
max_capacity = 100
# Time to live for cached items in seconds
time_to_live = 600

[engine]
# Default search engine (google, bing, duckduckgo)
favor = "google"
```

## Configuration Options

### Search Configuration (`[search]`)

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| user_agents | Array | [...] | List of user agents to rotate through |
| max_retries | Integer | 3 | Maximum number of retry attempts for failed requests |
| base_delay | Integer | 1000 | Base delay between requests in milliseconds |
| max_jitter | Integer | 1000 | Maximum random jitter added to delay in milliseconds |
| request_timeout | Integer | 10 | Request timeout in seconds |
| response_timeout | Integer | 10 | Response timeout in seconds |

### Cache Configuration (`[cache]`)

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| max_capacity | Integer | 100 | Maximum number of cached items |
| time_to_live | Integer | 600 | Time to live for cached items in seconds |

### Engine Configuration (`[engine]`)

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| favor | String | "google" | Default search engine (google, bing, duckduckgo) |

## Examples

### Minimal Configuration (only change search engine)
```toml
[engine]
favor = "bing"
```

### Cache-focused Configuration
```toml
[cache]
max_capacity = 1000
time_to_live = 3600
```

### Network-optimized Configuration
```toml
[search]
max_retries = 5
base_delay = 2000
max_jitter = 1000
request_timeout = 30
response_timeout = 30
```

## Configuration Tips

1. All configurations are optional - only specify what you want to customize
2. Default values will be used for any unspecified options
3. You can mix and match any configuration sections
4. Values specified in the config file will override default values

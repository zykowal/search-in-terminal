# Configuration Guide

## Table of Contents
- [Configuration File Location](#configuration-file-location)
- [Configuration Format](#configuration-format)
- [Configuration Options](#configuration-options)
  - [Search Configuration](#search-configuration-search)
  - [Cache Configuration](#cache-configuration-cache)
  - [Engine Configuration](#engine-configuration-engine)
- [Examples](#examples)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)
- [FAQ](#faq)

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
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:122.0) Gecko/20100101 Firefox/122.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2.1 Safari/605.1.15",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
]
# Maximum number of retry attempts for failed requests
max_retries = 3
# Base delay between requests in milliseconds
base_delay = 1000
# Maximum random jitter added to delay in milliseconds
max_jitter = 1000
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

| Option | Type | Default | Description | Example |
|--------|------|---------|-------------|---------|
| user_agents | Array | [...] | List of user agents to rotate through | See above |
| max_retries | Integer | 3 | Maximum number of retry attempts for failed requests | `5` |
| base_delay | Integer | 1000 | Base delay between requests in milliseconds | `2000` |
| max_jitter | Integer | 1000 | Maximum random jitter added to delay in milliseconds | `500` |
| request_timeout | Integer | 10 | Request timeout in seconds | `30` |
| response_timeout | Integer | 10 | Response timeout in seconds | `30` |

### Cache Configuration (`[cache]`)

| Option | Type | Default | Description | Example |
|--------|------|---------|-------------|---------|
| max_capacity | Integer | 100 | Maximum number of cached items | `1000` |
| time_to_live | Integer | 600 | Time to live for cached items in seconds | `3600` |

### Engine Configuration (`[engine]`)

| Option | Type | Default | Description | Example |
|--------|------|---------|-------------|---------|
| favor | String | "google" | Default search engine (google, bing, duckduckgo) | `"bing"` |

## Examples

### Minimal Configuration (only change search engine)
```toml
[engine]
favor = "bing"
```

### Cache-focused Configuration
```toml
[cache]
max_capacity = 1000  # Store more results
time_to_live = 3600  # Keep results for 1 hour
```

### Network-optimized Configuration
```toml
[search]
max_retries = 5          # More retries for unstable connections
base_delay = 2000        # Longer delay between requests
max_jitter = 1000        # Add randomness to delays
request_timeout = 30     # Longer timeout for slow connections
response_timeout = 30    # Longer timeout for slow responses
```

### Privacy-focused Configuration
```toml
[search]
user_agents = [
    "Mozilla/5.0 (Windows NT 10.0; rv:102.0) Gecko/20100101 Firefox/102.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:102.0) Gecko/20100101 Firefox/102.0"
]
base_delay = 2000        # Slower requests to avoid detection
max_jitter = 1500        # More random delays

[engine]
favor = "duckduckgo"     # Privacy-focused search engine
```

## Best Practices

1. **User Agents**
   - Use recent browser versions
   - Include a variety of browsers and platforms
   - Update user agents periodically

2. **Network Settings**
   - Adjust timeouts based on your connection speed
   - Use higher delays for less stable connections
   - Add jitter to avoid detection

3. **Cache Settings**
   - Balance cache size with memory usage
   - Set TTL based on how fresh results need to be
   - Consider disk space for large cache sizes

4. **Search Engine**
   - Choose based on your region and needs
   - Consider privacy implications
   - Test different engines for best results

## Troubleshooting

### Common Issues

1. **Slow Searches**
   - Decrease `base_delay` and `max_jitter`
   - Reduce `request_timeout` and `response_timeout`
   - Check your internet connection

2. **Search Failures**
   - Increase `max_retries`
   - Update user agents
   - Check if search engine is accessible

3. **High Memory Usage**
   - Reduce `max_capacity` in cache settings
   - Decrease `time_to_live`
   - Clear cache manually if needed

## FAQ

### How do I add a custom search engine?
Currently not supported. Use one of the provided engines: google, bing, or duckduckgo.

### Why are my searches slow?
Check your network settings and try reducing delays. Also ensure your user agents are up-to-date.

### How can I disable caching?
Set `max_capacity = 0` in the cache configuration.

### Can I use proxies?
Not directly supported in the configuration. Use system-level proxy settings.

### How often should I update user agents?
Update them every few months or when you notice search failures.

### What's the recommended cache size?
Start with the default (100) and adjust based on your needs and available memory.

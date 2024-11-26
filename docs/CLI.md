# Command Line Interface Guide

## Basic Usage

```bash
st [OPTIONS] [QUERY]
```

## Options

| Option | Short | Long | Description |
|--------|-------|------|-------------|
| Help | `-h` | `--help` | Display help information |
| Version | `-V` | `--version` | Display version information |
| Engine | `-e` | `--engine` | Specify search engine (google, bing, duckduckgo) |
| Results | `-n` | `--num` | Number of results to display (default: 10) |
| Config | `-c` | `--config` | Path to custom config file |
| Debug | `-d` | `--debug` | Enable debug logging |
| Quiet | `-q` | `--quiet` | Suppress all output except results |

## Examples

1. Basic search:
   ```bash
   st "rust programming"
   ```

2. Search with specific engine:
   ```bash
   st -e google "rust programming"
   ```

3. Specify number of results:
   ```bash
   st -n 20 "rust programming"
   ```

4. Use custom config file:
   ```bash
   st -c ~/myconfig.toml "rust programming"
   ```

5. Debug mode:
   ```bash
   st -d "rust programming"
   ```

6. Combine multiple options:
   ```bash
   st -e bing -n 15 -q "rust programming"
   ```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `ST_CONFIG` | Path to config file | Platform-specific |
| `ST_ENGINE` | Default search engine | google |
| `ST_DEBUG` | Enable debug logging | false |
| `ST_QUIET` | Enable quiet mode | false |

## Exit Codes

| Code | Description |
|------|-------------|
| 0 | Success |
| 1 | General error |
| 2 | Configuration error |
| 3 | Network error |
| 4 | Invalid argument |

## Interactive Commands

While in the TUI:

| Key | Action |
|-----|--------|
| `↑/k` | Move up |
| `↓/j` | Move down |
| `Enter` | Open selected result |
| `q` | Quit |
| `?` | Show help |
| `/` | Search within results |
| `n` | Next search result |
| `N` | Previous search result |
| `o` | Open in browser |
| `y` | Copy URL to clipboard |
| `r` | Refresh results |
| `f` | Toggle full URL display |
| `t` | New tab (if supported) |

## Configuration Integration

The CLI options take precedence over configuration file settings. The precedence order is:

1. Command line arguments
2. Environment variables
3. Configuration file
4. Default values

## Advanced Usage

### Search Operators

- `"exact phrase"`: Search for an exact phrase
- `site:example.com`: Limit search to a specific site
- `-exclude`: Exclude terms from search
- `filetype:pdf`: Search for specific file types

Example:
```bash
st 'site:github.com "rust programming" -beginner filetype:md'
```

### Output Formatting

Use the quiet mode with grep:
```bash
st -q "rust programming" | grep "tutorial"
```

### Multiple Searches

Chain searches with different engines:
```bash
st -e google "rust" && st -e bing "rust"
```

## Troubleshooting

1. If search fails:
   - Check internet connection
   - Verify config file syntax
   - Enable debug mode for detailed logs

2. If results are blocked:
   - Check rate limiting settings
   - Verify user agent configuration
   - Try a different search engine

3. If TUI doesn't display correctly:
   - Check terminal capabilities
   - Verify terminal size
   - Update terminal emulator

## See Also

- [Configuration Guide](CONFIG.md)
- [API Documentation](API.md)
- [Contributing Guide](../CONTRIBUTING.md)

# Search in Terminal 🔍

English | [简体中文](README.zh-CN.md) | [日本語](README.ja.md)

[![Crates.io](https://img.shields.io/crates/v/search-in-terminal.svg)](https://crates.io/crates/search-in-terminal)
[![Downloads](https://img.shields.io/crates/d/search-in-terminal.svg)](https://crates.io/crates/search-in-terminal)
[![License](https://img.shields.io/crates/l/search-in-terminal.svg)](LICENSE)

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Keyboard Shortcuts](#keyboard-shortcuts)
- [Configuration](#configuration)
- [Development](#development)
  - [Requirements](#requirements)
  - [Build](#build)
- [FAQ](#faq)
- [Contributing](#contributing)
- [Security](#security)
- [License](#license)
- [Author](#author)

A powerful terminal-based search tool that allows you to search and browse web content directly in your terminal. Supports multiple search engines (Google, Bing, DuckDuckGo) with an elegant TUI interface and convenient keyboard controls.

### 🎯 Why Search in Terminal?

- **Efficiency**: Search without leaving your terminal
- **Privacy**: Control your user agent and search behavior
- **Customization**: Configure search engines and behavior to your needs
- **Speed**: Fast and lightweight, with caching support
- **Cross-platform**: Works on Linux, macOS, and Windows

## Features ✨

- Multiple Search Engines Support 🌐
  - Google
  - Bing
  - DuckDuckGo
- Quick Engine Switching 🔄
- Search Results Caching 💾
- Beautiful TUI Interface 🎨
- Convenient Keyboard Controls ⌨️
- One-click Browser Opening 🚀

## Screenshots 📸

_Coming soon_

## Installation 📦

### Via Cargo

```bash
cargo install search-in-terminal
```

### From Source

```bash
git clone https://github.com/zykowal/search-in-terminal.git
cd search-in-terminal
cargo install --path .
```

## Usage 🛠️

After installation, simply run:

```bash
st
```

### Keyboard Shortcuts ⌨️

- `i`: Input mode
- `<C-u>`: Clear input
- `Esc`: Exit input mode
- `Enter`: Execute search when in input mode
- `e`: Switch search engine
- `↑/↓`: Browse search results
- `k/j`: Browse search results
- `Enter`: Open selected result in browser
- `q`: Quit program

## Configuration ⚙️

Configuration file location:
- Linux/macOS: `~/.config/st/config.toml`
- Windows: `%APPDATA%\st\config.toml`

For detailed configuration options, please see:
- [Configuration Guide](docs/CONFIG.md)
- [API Documentation](docs/API.md)
- [CLI Documentation](docs/CLI.md)

## Development 👨‍💻

### Requirements

- Rust 2021 edition

### Build

```bash
git clone https://github.com/zykowal/search-in-terminal.git
cd search-in-terminal
cargo build --release
```

## FAQ 💭

### How do I add a custom search engine?

Edit your config file and add a new entry under the `[search.engines]` section. See the [Configuration Guide](docs/CONFIG.md) for details.

### How do I change the default search engine?

Set the `default_engine` option in your config file. See the [Configuration Guide](docs/CONFIG.md) for details.

### Does it work with proxies?

Sorry! Temporarily unsupported. Use system-level proxy settings.

## Contributing 🤝

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License 📝

MIT License

## Author 👤

zykowal

## Changelog 📋

See [CHANGELOG.md](CHANGELOG.md) for release details.

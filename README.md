# Search in Terminal

English | [简体中文](README.zh-CN.md)

A powerful terminal-based search tool that allows you to search and browse web content directly in your terminal. Supports multiple search engines (Google, Bing, DuckDuckGo) with an elegant TUI interface and convenient keyboard controls.

## ✨ Features

- 🔍 Multiple Search Engines Support
  - Google
  - Bing
  - DuckDuckGo
- 🚀 Quick Engine Switching
- 💾 Search Results Caching
- 🎨 Beautiful TUI Interface
- ⌨️ Convenient Keyboard Controls
- 🌐 One-click Browser Opening

## 📦 Installation

```bash
cargo install search-in-terminal
```

## 🚀 Usage

After installation, simply run:

```bash
st
```

### ⌨️ Keyboard Shortcuts

- `Enter`: Execute search
- `Tab`: Switch search engine
- `↑/↓`: Browse search results
- `o`: Open selected result in browser
- `q`: Quit program

## ⚙️ Configuration

Configuration file location:
- Linux/macOS: `~/.config/st/config.toml`
- Windows: `%APPDATA%\st\config.toml`

## 🛠 Development

### Requirements

- Rust 2021 edition
- Cargo

### Dependencies

- ratatui: TUI interface
- crossterm: Terminal control
- tokio: Async runtime
- reqwest: HTTP client
- scraper: HTML parsing

### Local Build

```bash
git clone https://github.com/zykowal/search-in-terminal
cd search-in-terminal
cargo build --release
```

## 📝 License

MIT License

## 👨‍💻 Author

zykowal

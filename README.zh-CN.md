# Search in Terminal

[English](README.md) | 简体中文

一个强大的终端搜索工具，让你在终端中直接搜索并浏览网页内容。支持 Google、Bing、DuckDuckGo 等多个搜索引擎，提供优雅的 TUI 界面和便捷的键盘操作。

## ✨ 特性

- 🔍 支持多个搜索引擎
  - Google
  - Bing
  - DuckDuckGo
- 🚀 快速切换搜索引擎
- 💾 搜索结果缓存
- 🎨 美观的 TUI 界面
- ⌨️ 便捷的键盘操作
- 🌐 一键在浏览器中打开搜索结果

## 📦 安装

```bash
cargo install search-in-terminal
```

## 🚀 使用方法

直接运行：

```bash
st
```

### ⌨️ 快捷键

- `Enter`: 执行搜索
- `Tab`: 切换搜索引擎
- `↑/↓`: 浏览搜索结果
- `o`: 在浏览器中打开选中的结果
- `q`: 退出程序

## ⚙️ 配置

配置文件位置：
- Linux/macOS: `~/.config/st/config.toml`
- Windows: `%APPDATA%\st\config.toml`

## 🛠 开发

### 环境要求

- Rust 2021 edition
- Cargo

### 依赖

- ratatui: TUI 界面
- crossterm: 终端控制
- tokio: 异步运行时
- reqwest: HTTP 客户端
- scraper: HTML 解析

### 本地构建

```bash
git clone https://github.com/zykowal/search-in-terminal
cd search-in-terminal
cargo build --release
```

## 📝 许可证

MIT License

## 👨‍💻 作者

zykowal

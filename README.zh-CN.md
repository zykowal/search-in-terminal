# Search in Terminal 🔍

[English](README.md) | 简体中文 | [日本語](README.ja.md)

## 目录
- [特性](#特性)
- [安装](#安装)
- [使用方法](#使用方法)
  - [快捷键](#快捷键)
- [配置](#配置)
- [开发](#开发)
  - [环境要求](#环境要求)
  - [构建](#构建)
- [许可证](#许可证)
- [作者](#作者)

一个强大的终端搜索工具，让你在终端中直接搜索并浏览网页内容。支持 Google、Bing、DuckDuckGo 等多个搜索引擎，提供优雅的 TUI 界面和便捷的键盘操作。

## 特性 ✨

- 支持多个搜索引擎 🌐
  - Google
  - Bing
  - DuckDuckGo
- 快速切换搜索引擎 🔄
- 搜索结果缓存 💾
- 美观的 TUI 界面 🎨
- 便捷的键盘操作 ⌨️
- 一键在浏览器中打开搜索结果 🚀

## 安装 📦

```bash
cargo install search-in-terminal
```

## 使用方法 🛠️

直接运行：

```bash
st
```

### 快捷键 ⌨️

- `i`: 进入输入模式
- `<C-u>`: 清空输入
- `Esc`: 退出输入模式
- `Enter`: 在输入模式下执行搜索
- `e`: 切换搜索引擎
- `↑/↓`: 浏览搜索结果
- `k/j`: 浏览搜索结果
- `Enter`: 在浏览器中打开选中的结果
- `q`: 退出程序

## 配置 ⚙️

配置文件位置：
- Linux/macOS: `~/.config/st/config.toml`
- Windows: `%APPDATA%\st\config.toml`

详细配置选项请参考[配置指南](docs/CONFIG.md)。

### 是否支持代理？

抱歉！暂时不支持。请使用系统级代理设置。

## 开发 👨‍💻

### 环境要求

- Rust 2021 edition

### 构建

```bash
git clone https://github.com/zykowal/search-in-terminal.git
cd search-in-terminal
cargo build --release
```

## 贡献 🤝

欢迎贡献！详情请参阅我们的[贡献指南](CONTRIBUTING.md)。
Fork 本仓库，然后在本地进行修改，提交 Pull Request。
Fork 本仓库，创建属于自己仓库。

## 许可证 📝

MIT License

## 作者 👤

zykowal

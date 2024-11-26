# Search in Terminal 🔍

[English](README.md) | [简体中文](README.zh-CN.md) | 日本語

## 目次
- [特徴](#特徴)
- [インストール](#インストール)
- [使用方法](#使用方法)
  - [キーボードショートカット](#キーボードショートカット)
- [設定](#設定)
- [開発](#開発)
  - [必要環境](#必要環境)
  - [ビルド](#ビルド)
- [ライセンス](#ライセンス)
- [作者](#作者)

ターミナルから直接ウェブ検索ができる強力なツールです。Google、Bing、DuckDuckGoなどの検索エンジンをサポートし、エレガントなTUIインターフェースと便利なキーボード操作を提供します。

## 特徴 ✨

- 複数の検索エンジンをサポート 🌐
  - Google
  - Bing
  - DuckDuckGo
- 検索エンジンの素早い切り替え 🔄
- 検索結果のキャッシュ 💾
- 美しいTUIインターフェース 🎨
- 便利なキーボードショートカット ⌨️
- ワンクリックでブラウザで開く 🚀

## インストール 📦

```bash
cargo install search-in-terminal
```

## 使用方法 🛠️

実行するには：

```bash
st
```

### キーボードショートカット ⌨️

- `i`: 入力モードに入る
- `<C-u>`: 入力をクリア
- `Esc`: 入力モードを終了
- `Enter`: 入力モード中に検索を実行
- `e`: 検索エンジンを切り替え
- `↑/↓`: 検索結果をブラウズ
- `k/j`: 検索結果をブラウズ
- `Enter`: 選択した結果をブラウザで開く
- `q`: プログラムを終了

## 設定 ⚙️

設定ファイルの場所：
- Linux/macOS: `~/.config/st/config.toml`
- Windows: `%APPDATA%\st\config.toml`

詳細な設定オプションについては、[設定ガイド](docs/CONFIG.md)をご覧ください。

## 開発 👨‍💻

### 必要環境

- Rust 2021 edition

### ビルド

```bash
git clone https://github.com/zykowal/search-in-terminal.git
cd search-in-terminal
cargo build --release
```

## コントリビューション 🤝

コントリビューションを歓迎します！詳細は[コントリビューションガイド](CONTRIBUTING.md)をご覧ください。

### プロキシは使えますか？

申し訳ありません！現在は対応していません。システムレベルのプロキシ設定をご利用ください。

## ライセンス 📝

MIT ライセンス

## 作者 👤

zykowal

# yt-dlp Modern GUI


yt-dlp を使用してビデオをダウンロードするための最新のクロスプラットフォーム対応デスクトップアプリケーションです。
Tauri 2.0（Rust）と SvelteKit で構築された、ビデオダウンロードを管理するための清潔で直感的なインターフェースを提供します。

[**English**](../README.md) | [**한국어**](README.ko.md) | **日本語** | [**中文(简体)**](README.zh-CN.md) | [**中文(繁體)**](README.zh-TW.md) | [**Español**](README.es.md) | [**Français**](README.fr.md) | [**Deutsch**](README.de.md) | [**Português**](README.pt-BR.md) | [**Русский**](README.ru.md) | [**Tiếng Việt**](README.vi.md)

## スクリーンショット

<p align="center">
  <img src="App.png" alt="yt-dlp Modern GUI" width="450">
</p>
<p align="center">
  <img src="Downloading.png" alt="yt-dlp Modern GUI" width="450">
</p>

## 機能

- フォーマットと品質を選択してビデオとプレイリストをダウンロード
- キャンセル、再試行機能付きの並行ダウンロードキュー
- 検索と管理機能付きのダウンロード履歴
- yt-dlp と FFmpeg の自動依存関係検出とインストールガイド
- ファイル名テンプレートのカスタマイズ（シンプルモードと詳細モード）
- 認証コンテンツ用のクッキーサポート
- 重複ダウンロード検出
- 多言語対応（English、한국어、日本語、简体中文、繁體中文、Français、Deutsch）
- 4つのカラーテーマ（Dark、Violet、Red、Light）
- クロスプラットフォーム対応（Windows、macOS、Linux）

## ソースからビルド

### 前提条件

- [Rust](https://www.rust-lang.org/tools/install)（最新の stable バージョン）
- [Node.js](https://nodejs.org/)（v18+）
- [Bun](https://bun.sh/)（パッケージマネージャー）
- [Tauri 2.0](https://v2.tauri.app/start/prerequisites/) のプラットフォーム別依存関係

### ビルド手順

```bash
# リポジトリをクローン
git clone https://github.com/shlifedev/yt-dlp-modern-gui.git
cd yt-dlp-modern-gui

# フロントエンドの依存関係をインストール
bun install

# 開発モードで実行
bun run tauri dev

# プロダクションビルド
bun run tauri build
```

プロダクションビルドの出力は `src-tauri/target/release/bundle/` に生成されます。

## ロードマップ

1. モバイルユーザー向けダウンローダーアプリ（yt-dlpサーバーを自分でホスティングできます）
2. バージョンアップデーター

## ライセンス

このプロジェクトは [MIT License](../LICENSE) の下でライセンスされています。

# yt-dlp Modern GUI

一個現代化、跨平台的桌面應用程式，用於使用 yt-dlp 下載影片。
採用 Tauri 2.0（Rust）和 SvelteKit 構建，提供乾淨直觀的介面來管理影片下載。

[**English**](../README.md) | [**한국어**](README.ko.md) | [**日本語**](README.ja.md) | [**中文(简体)**](README.zh-CN.md) | [**中文(繁體)**](README.zh-TW.md) | [**Español**](README.es.md) | [**Français**](README.fr.md) | [**Deutsch**](README.de.md) | [**Português**](README.pt-BR.md) | [**Русский**](README.ru.md) | [**Tiếng Việt**](README.vi.md)

## 功能

- 支援影片和播放清單下載，可選擇格式和畫質
- 並行下載佇列，支援暫停、取消和重試
- 下載歷史記錄，包含搜尋和管理功能
- 自動偵測和安裝 yt-dlp 和 FFmpeg 依賴項
- 檔案名稱樣板自訂（簡易和進階模式）
- Cookie 支援以下載需認證的內容
- 重複下載偵測
- 跨平台支援（Windows、macOS、Linux）

## 技術棧

- **後端**：Tauri 2.0 / Rust
- **前端**：SvelteKit 2.x / Svelte 5 / Tailwind CSS v4 / Skeleton UI v4

## 授權

本專案採用 [MIT 授權](../LICENSE)。

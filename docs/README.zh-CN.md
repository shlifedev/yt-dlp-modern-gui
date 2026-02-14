# yt-dlp Modern GUI

一个现代化、跨平台的桌面应用，用于使用 yt-dlp 下载视频。
基于 Tauri 2.0 (Rust) 和 SvelteKit 构建，为视频下载管理提供了简洁直观的界面。

[**English**](../README.md) | [**한국어**](README.ko.md) | [**日本語**](README.ja.md) | **中文(简体)** | [**中文(繁體)**](README.zh-TW.md) | [**Español**](README.es.md) | [**Français**](README.fr.md) | [**Deutsch**](README.de.md) | [**Português**](README.pt-BR.md) | [**Русский**](README.ru.md) | [**Tiếng Việt**](README.vi.md)

## 功能特性

- 支持格式和画质选择的视频和播放列表下载
- 支持暂停、取消和重试的并发下载队列
- 带有搜索和管理功能的下载历史记录
- 自动 yt-dlp 和 FFmpeg 依赖检测和安装
- 文件名模板自定义（简洁和高级模式）
- 认证内容的 Cookie 支持
- 重复下载检测
- 跨平台支持（Windows、macOS、Linux）

## 技术栈

- **Backend**: Tauri 2.0 / Rust
- **Frontend**: SvelteKit 2.x / Svelte 5 / Tailwind CSS v4 / Skeleton UI v4

## 许可证

该项目在 [MIT License](../LICENSE) 下获得许可。

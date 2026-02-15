# yt-dlp Modern GUI


Eine moderne, plattformübergreifende Desktop-Anwendung zum Herunterladen von Videos mit yt-dlp.
Gebaut mit Tauri 2.0 (Rust) und SvelteKit, bietet eine saubere und intuitive Benutzeroberfläche zur Verwaltung von Video-Downloads.

[**English**](../README.md) | [**한국어**](README.ko.md) | [**日本語**](README.ja.md) | [**中文(简体)**](README.zh-CN.md) | [**中文(繁體)**](README.zh-TW.md) | [**Español**](README.es.md) | [**Français**](README.fr.md) | **Deutsch** | [**Português**](README.pt-BR.md) | [**Русский**](README.ru.md) | [**Tiếng Việt**](README.vi.md)

## Screenshots

<p align="center">
  <img src="App.png" alt="yt-dlp Modern GUI" width="450">
</p>
<p align="center">
  <img src="Downloading.png" alt="yt-dlp Modern GUI" width="450">
</p>

## Funktionen

- Video- und Playlist-Download mit Format- und Qualitätsauswahl
- Gleichzeitige Download-Warteschlange mit Abbruch- und Wiederholungsfunktion
- Download-Verlauf mit Suche und Verwaltung
- Automatische Erkennung von yt-dlp und FFmpeg Abhängigkeiten mit Installationsanleitung
- Anpassung der Dateinamenvorlage (einfacher & erweiterter Modus)
- Cookie-Unterstützung für authentifizierte Inhalte
- Duplikat-Download-Erkennung
- Mehrsprachige Unterstützung (English, 한국어, 日本語, 简体中文, 繁體中文, Français, Deutsch)
- 4 Farbthemen (Dark, Violet, Red, Light)
- Plattformübergreifende Unterstützung (Windows, macOS, Linux)

## Aus dem Quellcode bauen

### Voraussetzungen

- [Rust](https://www.rust-lang.org/tools/install) (neueste stable Version)
- [Node.js](https://nodejs.org/) (v18+)
- [Bun](https://bun.sh/) (Paketmanager)
- Plattformspezifische Abhängigkeiten für [Tauri 2.0](https://v2.tauri.app/start/prerequisites/)

### Schritte

```bash
# Repository klonen
git clone https://github.com/shlifedev/yt-dlp-modern-gui.git
cd yt-dlp-modern-gui

# Frontend-Abhängigkeiten installieren
bun install

# Im Entwicklungsmodus starten
bun run tauri dev

# Für Produktion bauen
bun run tauri build
```

Das Produktions-Build befindet sich in `src-tauri/target/release/bundle/`.

## Roadmap

1. Downloader-App für mobile Nutzer (Sie können Ihren eigenen yt-dlp-Server hosten)
2. Versions-Updater

## Lizenz

Dieses Projekt ist unter der [MIT-Lizenz](../LICENSE) lizenziert.

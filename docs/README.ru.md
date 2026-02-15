# yt-dlp Modern GUI


Современное кроссплатформенное настольное приложение для загрузки видео с использованием yt-dlp.
Построено на Tauri 2.0 (Rust) и SvelteKit, предоставляя чистый и интуитивный интерфейс для управления загрузками видео.

[**English**](../README.md) | [**한국어**](README.ko.md) | [**日本語**](README.ja.md) | [**中文(简体)**](README.zh-CN.md) | [**中文(繁體)**](README.zh-TW.md) | [**Español**](README.es.md) | [**Français**](README.fr.md) | [**Deutsch**](README.de.md) | [**Português**](README.pt-BR.md) | **Русский** | [**Tiếng Việt**](README.vi.md)

## Скриншоты

<p align="center">
  <img src="App.png" alt="yt-dlp Modern GUI" width="450">
</p>
<p align="center">
  <img src="Downloading.png" alt="yt-dlp Modern GUI" width="450">
</p>

## Возможности

- Загрузка видео и плейлистов с выбором формата и качества
- Очередь одновременных загрузок с отменой и повтором
- История загрузок с поиском и управлением
- Автоматическое обнаружение зависимостей yt-dlp и FFmpeg с руководством по установке
- Настройка шаблона имени файла (простой и продвинутый режимы)
- Поддержка файлов Cookie для аутентифицированного контента
- Обнаружение дублирующихся загрузок
- Многоязычная поддержка (English, 한국어, 日本語, 简体中文, 繁體中文, Français, Deutsch)
- 4 цветовые темы (Dark, Violet, Red, Light)
- Кроссплатформенная поддержка (Windows, macOS, Linux)

## Сборка из исходного кода

### Предварительные требования

- [Rust](https://www.rust-lang.org/tools/install) (последняя stable версия)
- [Node.js](https://nodejs.org/) (v18+)
- [Bun](https://bun.sh/) (менеджер пакетов)
- Платформо-зависимые зависимости для [Tauri 2.0](https://v2.tauri.app/start/prerequisites/)

### Шаги

```bash
# Клонировать репозиторий
git clone https://github.com/shlifedev/yt-dlp-modern-gui.git
cd yt-dlp-modern-gui

# Установить зависимости фронтенда
bun install

# Запустить в режиме разработки
bun run tauri dev

# Сборка для продакшена
bun run tauri build
```

Результат продакшен-сборки находится в `src-tauri/target/release/bundle/`.

## Планы на будущее

1. Приложение для загрузки для мобильных пользователей (можно разместить собственный сервер yt-dlp)
2. Обновление версий

## Лицензия

Этот проект распространяется под лицензией [MIT License](../LICENSE).

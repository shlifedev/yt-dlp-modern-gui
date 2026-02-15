# yt-dlp Modern GUI


Una aplicación de escritorio moderna y multiplataforma para descargar videos usando yt-dlp.
Construida con Tauri 2.0 (Rust) y SvelteKit, proporcionando una interfaz limpia e intuitiva para administrar descargas de videos.

[**English**](../README.md) | [**한국어**](README.ko.md) | [**日本語**](README.ja.md) | [**中文(简体)**](README.zh-CN.md) | [**中文(繁體)**](README.zh-TW.md) | [**Français**](README.fr.md) | [**Deutsch**](README.de.md) | [**Português**](README.pt-BR.md) | [**Русский**](README.ru.md) | [**Tiếng Việt**](README.vi.md) | **Español**

## Capturas de pantalla

<p align="center">
  <img src="App.png" alt="yt-dlp Modern GUI" width="450">
</p>
<p align="center">
  <img src="Downloading.png" alt="yt-dlp Modern GUI" width="450">
</p>

## Características

- Descarga de videos y listas de reproducción con selección de formato y calidad
- Cola de descargas concurrentes con cancelación y reintentos
- Historial de descargas con búsqueda y administración
- Detección automática de dependencias yt-dlp y FFmpeg con guía de instalación
- Personalización de plantillas de nombres de archivo (modo simple y avanzado)
- Soporte para cookies para contenido autenticado
- Detección de descargas duplicadas
- Soporte multiidioma (English, 한국어, 日本語, 简体中文, 繁體中文, Français, Deutsch)
- 4 temas de colores (Dark, Violet, Red, Light)
- Soporte multiplataforma (Windows, macOS, Linux)

## Compilar desde el código fuente

### Requisitos previos

- [Rust](https://www.rust-lang.org/tools/install) (última versión stable)
- [Node.js](https://nodejs.org/) (v18+)
- [Bun](https://bun.sh/) (gestor de paquetes)
- Dependencias específicas de plataforma para [Tauri 2.0](https://v2.tauri.app/start/prerequisites/)

### Pasos

```bash
# Clonar el repositorio
git clone https://github.com/shlifedev/yt-dlp-modern-gui.git
cd yt-dlp-modern-gui

# Instalar dependencias del frontend
bun install

# Ejecutar en modo desarrollo
bun run tauri dev

# Compilar para producción
bun run tauri build
```

La compilación de producción se genera en `src-tauri/target/release/bundle/`.

## Hoja de ruta

1. Aplicación de descarga para usuarios móviles (puede alojar su propio servidor yt-dlp)
2. Actualizador de versiones

## Licencia

Este proyecto está bajo la licencia [MIT License](../LICENSE).

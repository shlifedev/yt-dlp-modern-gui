# yt-dlp Modern GUI


yt-dlp를 사용하여 비디오를 다운로드하기 위한 현대적이고 크로스 플랫폼 데스크톱 애플리케이션입니다.
Tauri 2.0 (Rust)과 SvelteKit으로 구축되어 비디오 다운로드를 관리하기 위한 깔끔하고 직관적인 인터페이스를 제공합니다.

[**English**](../README.md) | **한국어** | [**日本語**](README.ja.md) | [**中文(简体)**](README.zh-CN.md) | [**中文(繁體)**](README.zh-TW.md) | [**Español**](README.es.md) | [**Français**](README.fr.md) | [**Deutsch**](README.de.md) | [**Português**](README.pt-BR.md) | [**Русский**](README.ru.md) | [**Tiếng Việt**](README.vi.md)

## 스크린샷

<p align="center">
  <img src="App.png" alt="yt-dlp Modern GUI" width="450">
</p>
<p align="center">
  <img src="Downloading.png" alt="yt-dlp Modern GUI" width="450">
</p>

## 기능

- 형식 및 화질 선택을 통한 비디오 및 플레이리스트 다운로드
- 취소 및 재시도 기능이 있는 동시 다운로드 큐
- 검색 및 관리 기능이 있는 다운로드 히스토리
- 자동 yt-dlp 및 FFmpeg 의존성 감지 및 설치 안내
- 파일명 템플릿 커스터마이징 (간단한 모드 & 고급 모드)
- 인증된 콘텐츠를 위한 쿠키 지원
- 중복 다운로드 감지
- 다국어 지원 (English, 한국어, 日本語, 简体中文, 繁體中文, Français, Deutsch)
- 4가지 컬러 테마 (Dark, Violet, Red, Light)
- 크로스 플랫폼 지원 (Windows, macOS, Linux)

## 직접 빌드하기

### 사전 준비

- [Rust](https://www.rust-lang.org/tools/install) (최신 stable 버전)
- [Node.js](https://nodejs.org/) (v18+)
- [Bun](https://bun.sh/) (패키지 매니저)
- [Tauri 2.0](https://v2.tauri.app/start/prerequisites/) 플랫폼별 의존성

### 빌드 방법

```bash
# 저장소 클론
git clone https://github.com/shlifedev/yt-dlp-modern-gui.git
cd yt-dlp-modern-gui

# 프론트엔드 의존성 설치
bun install

# 개발 모드 실행
bun run tauri dev

# 프로덕션 빌드
bun run tauri build
```

프로덕션 빌드 결과물은 `src-tauri/target/release/bundle/`에 생성됩니다.

## 앞으로의 계획

1. 모바일 앱 사용자를 위한 다운로더 앱 (직접 yt-dlp 서버를 호스팅할 수 있습니다)
2. 버전 업데이터

## 라이선스

이 프로젝트는 [MIT License](../LICENSE)에 따라 라이선스됩니다.

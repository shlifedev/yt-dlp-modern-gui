# Sprite Sheet Describer - Design Document

**Date:** 2026-02-02
**Status:** Approved
**Branch:** feat/sprite-sheet-describer

## Overview

스프라이트 시트를 불러와서 n×n 그리드로 나누고, 각 셀에 대한 설명을 입력할 수 있는 도구입니다. 반복적인 입력을 줄이기 위해 프리셋 태그, 멀티 셀 선택, 커스텀 태그 기능을 제공합니다.

## User Requirements

- 스프라이트 시트 이미지 로드 (파일 선택 버튼)
- 사용자 지정 그리드 크기 설정 (행/열 입력)
- 각 셀에 설명 입력 (모달 팝업)
- 프리셋 태그로 빠른 입력
- 멀티 셀 선택으로 일괄 입력
- 커스텀 태그 저장 및 재사용
- 출력 형식 선택 (Row/Col vs Index)
- JSON 및 텍스트 리스트 출력

## Architecture

### 1. Page Structure

```
/src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte
```

**Layout:**
- Header (제목, 설명)
- Control Panel (파일 선택, 그리드 설정, 선택 모드)
- Canvas Area (스프라이트 시트 + 그리드 오버레이)
- Quick Input Toolbar (프리셋 태그, 커스텀 태그)
- Output Area (JSON, 텍스트 리스트)
- Project Management (저장/불러오기, 초기화)

### 2. State Management (Svelte 5 Runes)

```typescript
let spriteImage = $state<HTMLImageElement | null>(null)
let gridRows = $state(4)
let gridCols = $state(4)
let cellDescriptions = $state<Map<string, string>>(new Map())
let selectedCells = $state<Set<string>>(new Set())
let selectionMode = $state<"single" | "multi">("single")
let showModal = $state(false)
let currentCell = $state<{row: number, col: number} | null>(null)
let customTags = $state<string[]>([])
let outputFormat = $state<"rowcol" | "index">("rowcol")
```

### 3. Data Structure

**Cell Key Format:** `"row-col"` (예: `"0-0"`, `"2-3"`)

**Index Calculation:** `index = row * gridCols + col`

### 4. Components

#### Canvas Grid System
- Canvas element로 이미지와 그리드 렌더링
- 그리드 라인: 반투명 흰색 (#ffffff40)
- 설명이 있는 셀: 파란색 반투명 배경 (rgba(14, 99, 156, 0.3))
- 선택된 셀: 파란색 테두리 (2px solid #0e639c)
- 호버 효과: 밝은 테두리

#### Selection Modes

**Single Mode:**
- 클릭 → 모달 열기
- 기존 설명 있으면 미리 채우기

**Multi Mode:**
- Shift + 클릭: 사각형 영역 선택
- Ctrl/Cmd + 클릭: 개별 셀 토글
- 선택된 셀 개수 표시

#### Modal Popup
- 제목: "셀 설명 편집 (행: X, 열: Y, 인덱스: Z)"
- Textarea (여러 줄 입력)
- 빠른 선택: 프리셋 태그 버튼들
- 버튼: 저장, 삭제, 취소
- ESC / 외부 클릭으로 닫기

### 5. Preset Tags

타일셋/스프라이트에서 자주 사용되는 설명:

**지형:** 벽(위), 벽(아래), 벽(좌), 벽(우), 바닥, 천장
**구조물:** 문, 창문, 계단, 사다리
**자연:** 풀, 나무, 물, 돌
**상호작용:** 상자, 스위치, 레버
**캐릭터:** idle, walk, run, jump, attack

### 6. Custom Tags

- localStorage에 저장: `localStorage.getItem/setItem("customTags")`
- 추가: 모달 또는 인라인 입력
- 삭제: 각 태그에 × 버튼
- 시각적 구분: 초록색 배경 (#4CAF50)

### 7. Output Formats

#### JSON (Row/Col Mode)
```json
{
  "gridSize": { "rows": 4, "cols": 4 },
  "format": "rowcol",
  "cells": [
    { "row": 0, "col": 0, "description": "벽(위)" },
    { "row": 0, "col": 1, "description": "벽(위)" }
  ]
}
```

#### JSON (Index Mode)
```json
{
  "gridSize": { "rows": 4, "cols": 4 },
  "format": "index",
  "cells": [
    { "index": 0, "description": "벽(위)" },
    { "index": 1, "description": "벽(위)" }
  ]
}
```

#### Text List (Row/Col)
```
[0,0] 벽(위)
[0,1] 벽(위)
[1,0] 문
```

#### Text List (Index)
```
[0] 벽(위)
[1] 벽(위)
[4] 문
```

### 8. Project Save/Load

**Save Format:**
```json
{
  "imageName": "spritesheet.png",
  "imageDataUrl": "data:image/png;base64,...",
  "gridSize": { "rows": 4, "cols": 4 },
  "descriptions": { "0-0": "벽(위)", "0-1": "벽(아래)" },
  "customTags": ["특수타일", "보스방"]
}
```

- 저장: JSON 파일 다운로드
- 불러오기: 파일 선택 → 상태 복원

### 9. Error Handling

- 이미지 로드 실패 → 에러 메시지 표시
- 그리드 크기 제한: 1 ≤ rows/cols ≤ 50
- 잘못된 프로젝트 파일 → 에러 메시지

## User Flow

1. 파일 선택 버튼으로 스프라이트 시트 이미지 로드
2. 행/열 입력으로 그리드 크기 설정
3. 단일/멀티 선택 모드 선택
4. 셀 클릭하여 설명 입력 또는 프리셋 태그 사용
5. 필요시 커스텀 태그 추가
6. 출력 형식(Row/Col vs Index) 선택
7. JSON 또는 텍스트 리스트 복사
8. (선택) 프로젝트 저장

## Technical Details

### Canvas Rendering

```typescript
function drawGrid() {
  if (!canvas || !spriteImage) return
  const ctx = canvas.getContext("2d")!

  // Clear and draw image
  ctx.clearRect(0, 0, canvas.width, canvas.height)
  ctx.drawImage(spriteImage, 0, 0, canvas.width, canvas.height)

  const cellWidth = canvas.width / gridCols
  const cellHeight = canvas.height / gridRows

  // Draw grid lines
  ctx.strokeStyle = "#ffffff40"
  ctx.lineWidth = 1

  for (let i = 0; i <= gridRows; i++) {
    ctx.beginPath()
    ctx.moveTo(0, i * cellHeight)
    ctx.lineTo(canvas.width, i * cellHeight)
    ctx.stroke()
  }

  for (let i = 0; i <= gridCols; i++) {
    ctx.beginPath()
    ctx.moveTo(i * cellWidth, 0)
    ctx.lineTo(i * cellWidth, canvas.height)
    ctx.stroke()
  }

  // Highlight cells with descriptions
  cellDescriptions.forEach((desc, key) => {
    const [row, col] = key.split("-").map(Number)
    ctx.fillStyle = "rgba(14, 99, 156, 0.3)"
    ctx.fillRect(col * cellWidth, row * cellHeight, cellWidth, cellHeight)
  })

  // Highlight selected cells
  selectedCells.forEach(key => {
    const [row, col] = key.split("-").map(Number)
    ctx.strokeStyle = "#0e639c"
    ctx.lineWidth = 2
    ctx.strokeRect(col * cellWidth, row * cellHeight, cellWidth, cellHeight)
  })
}
```

### Cell Click Handler

```typescript
function handleCanvasClick(e: MouseEvent) {
  if (!canvas || !spriteImage) return

  const rect = canvas.getBoundingClientRect()
  const x = e.clientX - rect.left
  const y = e.clientY - rect.top

  const cellWidth = canvas.width / gridCols
  const cellHeight = canvas.height / gridRows

  const col = Math.floor(x / cellWidth)
  const row = Math.floor(y / cellHeight)
  const key = `${row}-${col}`

  if (selectionMode === "single") {
    currentCell = { row, col }
    showModal = true
  } else {
    // Multi mode
    if (e.shiftKey) {
      // Rectangle selection
      selectRectangle(lastSelectedCell, { row, col })
    } else if (e.ctrlKey || e.metaKey) {
      // Toggle selection
      if (selectedCells.has(key)) {
        selectedCells.delete(key)
      } else {
        selectedCells.add(key)
      }
      selectedCells = new Set(selectedCells)
    }
    lastSelectedCell = { row, col }
  }

  drawGrid()
}
```

## Implementation Tasks

1. Create route: `/src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte`
2. Update `src/lib/config/tools.ts` to add new tool
3. Implement canvas grid system
4. Implement single/multi selection modes
5. Create modal component for cell description
6. Implement preset tags
7. Implement custom tags with localStorage
8. Implement output format toggle
9. Implement JSON/text list generation
10. Implement project save/load
11. Add error handling and validation
12. Style with existing design system

## Design Decisions

- **Canvas vs SVG:** Canvas를 선택한 이유는 대용량 이미지와 많은 그리드 셀을 효율적으로 렌더링하기 위함
- **Map vs Object:** cellDescriptions를 Map으로 선택한 이유는 동적 키 추가/삭제가 빈번하기 때문
- **localStorage vs IndexedDB:** customTags는 작은 데이터이므로 localStorage로 충분
- **Modal vs Inline:** 모달을 선택한 이유는 UI가 깨끗하고 직관적이며, 여러 줄 입력에 적합

## Future Enhancements (Not in scope)

- 자동 그리드 감지 (이미지 분석)
- 프리셋 태그 카테고리 커스터마이징
- 셀 복사/붙여넣기
- 되돌리기/다시하기 (Undo/Redo)
- 이미지 줌/팬
- 셀 병합
- LLM API 연동으로 자동 설명 생성

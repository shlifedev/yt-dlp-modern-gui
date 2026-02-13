# Sprite Sheet Describer Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a tool to annotate sprite sheet images with an n×n grid overlay where users can describe each cell, supporting preset tags, multi-selection, and multiple output formats.

**Architecture:** Single-page Svelte 5 component using Canvas for grid rendering, Map for cell data storage, localStorage for custom tags, and pure frontend implementation (no Rust backend needed).

**Tech Stack:** SvelteKit, Svelte 5 runes, HTML Canvas API, localStorage

---

## Task 1: Update tools config and create basic page structure

**Files:**
- Modify: `src/lib/config/tools.ts`
- Create: `src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte`

**Step 1: Add new tool to config**

Edit `src/lib/config/tools.ts`:

```typescript
export const toolCategories: ToolCategory[] = [
  {
    id: "encoding",
    name: "인코딩",
    tools: [
      { id: "base64", name: "Base64", path: "/tools/encoding/base64" }
    ]
  },
  {
    id: "llm-toolset",
    name: "LLM 도구",
    tools: [
      { id: "motion-descriptor", name: "Motion Descriptor", path: "/tools/llm-toolset/motion-descriptor" },
      { id: "sprite-sheet-describer", name: "Sprite Sheet Describer", path: "/tools/llm-toolset/sprite-sheet-describer" }
    ]
  }
]
```

**Step 2: Create basic page skeleton**

Create `src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte`:

```svelte
<script lang="ts">
  // State will be added in next task
</script>

<div class="tool-page">
  <header class="tool-header">
    <h1>Sprite Sheet Describer</h1>
    <p>스프라이트 시트를 그리드로 나누고 각 셀에 설명을 추가합니다.</p>
  </header>

  <div class="instructions">
    <h3>사용 방법</h3>
    <ol>
      <li>스프라이트 시트 이미지를 불러옵니다</li>
      <li>그리드 크기를 설정합니다 (행/열)</li>
      <li>셀을 클릭하여 설명을 입력하거나 프리셋 태그를 사용합니다</li>
      <li>JSON 또는 텍스트 리스트 형식으로 내보냅니다</li>
    </ol>
  </div>
</div>

<style>
  .tool-page {
    padding: 2rem;
    max-width: 1400px;
    margin: 0 auto;
  }

  .tool-header {
    margin-bottom: 1.5rem;
  }

  .tool-header h1 {
    margin: 0 0 0.5rem 0;
    font-size: 1.5rem;
  }

  .tool-header p {
    margin: 0;
    color: #888;
  }

  .instructions {
    background: #1e1e1e;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 1.5rem;
    margin-bottom: 2rem;
  }

  .instructions h3 {
    margin-top: 0;
    font-size: 1rem;
    color: #0e639c;
  }

  .instructions ol {
    margin: 0.5rem 0 0 1.5rem;
    padding: 0;
  }

  .instructions li {
    margin: 0.5rem 0;
    color: #ccc;
  }
</style>
```

**Step 3: Verify navigation works**

Run: `bun run dev`
Expected: Navigate to sidebar → LLM 도구 → Sprite Sheet Describer, see basic page

**Step 4: Commit**

```bash
git add src/lib/config/tools.ts src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte
git commit -m "feat: add sprite sheet describer tool skeleton

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 2: Add state management and control panel UI

**Files:**
- Modify: `src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte`

**Step 1: Add state declarations**

Add to top of `<script lang="ts">` section:

```typescript
  interface CellKey {
    row: number
    col: number
  }

  let spriteImage = $state<HTMLImageElement | null>(null)
  let imageFile = $state<File | null>(null)
  let gridRows = $state(4)
  let gridCols = $state(4)
  let cellDescriptions = $state<Map<string, string>>(new Map())
  let selectedCells = $state<Set<string>>(new Set())
  let selectionMode = $state<"single" | "multi">("single")
  let showModal = $state(false)
  let currentCell = $state<CellKey | null>(null)
  let customTags = $state<string[]>([])
  let outputFormat = $state<"rowcol" | "index">("rowcol")
  let canvas = $state<HTMLCanvasElement | null>(null)

  function cellKey(row: number, col: number): string {
    return `${row}-${col}`
  }

  function parseKey(key: string): CellKey {
    const [row, col] = key.split("-").map(Number)
    return { row, col }
  }

  function cellToIndex(row: number, col: number): number {
    return row * gridCols + col
  }
```

**Step 2: Add image loading function**

Add after state declarations:

```typescript
  function handleFileSelect(e: Event) {
    const input = e.target as HTMLInputElement
    const file = input.files?.[0]
    if (!file) return

    imageFile = file
    const reader = new FileReader()

    reader.onload = (evt) => {
      const img = new Image()
      img.onload = () => {
        spriteImage = img
        cellDescriptions.clear()
        selectedCells.clear()
        drawGrid()
      }
      img.src = evt.target?.result as string
    }

    reader.readAsDataURL(file)
  }

  function drawGrid() {
    // Placeholder - will implement in next task
    console.log("drawGrid called")
  }
```

**Step 3: Add control panel UI**

Add after instructions div, before closing `</div>`:

```svelte
  <div class="control-panel">
    <div class="control-section">
      <h3>이미지 불러오기</h3>
      <input
        type="file"
        accept="image/*"
        onchange={handleFileSelect}
        class="file-input"
      />
      {#if imageFile}
        <span class="file-name">{imageFile.name}</span>
      {/if}
    </div>

    <div class="control-section">
      <h3>그리드 설정</h3>
      <div class="grid-inputs">
        <label>
          행:
          <input
            type="number"
            min="1"
            max="50"
            bind:value={gridRows}
            onchange={() => drawGrid()}
          />
        </label>
        <label>
          열:
          <input
            type="number"
            min="1"
            max="50"
            bind:value={gridCols}
            onchange={() => drawGrid()}
          />
        </label>
      </div>
    </div>

    <div class="control-section">
      <h3>선택 모드</h3>
      <div class="mode-toggle">
        <button
          class:active={selectionMode === "single"}
          onclick={() => { selectionMode = "single"; selectedCells.clear() }}
        >
          단일 선택
        </button>
        <button
          class:active={selectionMode === "multi"}
          onclick={() => selectionMode = "multi"}
        >
          멀티 선택
        </button>
      </div>
      {#if selectionMode === "multi" && selectedCells.size > 0}
        <span class="selection-count">{selectedCells.size}개 셀 선택됨</span>
      {/if}
    </div>
  </div>
```

**Step 4: Add control panel styles**

Add to `<style>` section:

```css
  .control-panel {
    display: flex;
    gap: 2rem;
    margin-bottom: 2rem;
    flex-wrap: wrap;
  }

  .control-section {
    background: #1e1e1e;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 1rem;
  }

  .control-section h3 {
    margin: 0 0 0.75rem 0;
    font-size: 0.9rem;
    color: #0e639c;
  }

  .file-input {
    display: block;
    color: #ccc;
    font-size: 0.9rem;
  }

  .file-name {
    display: block;
    margin-top: 0.5rem;
    font-size: 0.85rem;
    color: #888;
  }

  .grid-inputs {
    display: flex;
    gap: 1rem;
  }

  .grid-inputs label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.9rem;
    color: #ccc;
  }

  .grid-inputs input {
    width: 60px;
    padding: 0.4rem;
    background: #0d0d0d;
    border: 1px solid #444;
    color: #e0e0e0;
    border-radius: 4px;
  }

  .mode-toggle {
    display: flex;
    gap: 0.5rem;
  }

  .mode-toggle button {
    padding: 0.5rem 1rem;
    background: #333;
    border: 1px solid #444;
    color: #ccc;
    cursor: pointer;
    border-radius: 4px;
    font-size: 0.85rem;
  }

  .mode-toggle button:hover {
    background: #444;
  }

  .mode-toggle button.active {
    background: #0e639c;
    border-color: #0e639c;
    color: #fff;
  }

  .selection-count {
    display: block;
    margin-top: 0.5rem;
    font-size: 0.85rem;
    color: #4CAF50;
  }
```

**Step 5: Test control panel**

Run: `bun run dev`
Expected: See control panel with file input, grid inputs, and mode toggle buttons

**Step 6: Commit**

```bash
git add src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte
git commit -m "feat: add state management and control panel UI

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 3: Implement canvas grid rendering

**Files:**
- Modify: `src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte`

**Step 1: Implement drawGrid function**

Replace the placeholder `drawGrid` function:

```typescript
  function drawGrid() {
    if (!canvas || !spriteImage) return

    const ctx = canvas.getContext("2d")
    if (!ctx) return

    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height)

    // Draw sprite sheet image
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
      const { row, col } = parseKey(key)
      ctx.fillStyle = "rgba(14, 99, 156, 0.3)"
      ctx.fillRect(col * cellWidth, row * cellHeight, cellWidth, cellHeight)

      // Draw checkmark for cells with descriptions
      ctx.fillStyle = "#4CAF50"
      ctx.font = "16px sans-serif"
      ctx.fillText("✓", col * cellWidth + 5, row * cellHeight + 20)
    })

    // Highlight selected cells (multi mode)
    selectedCells.forEach(key => {
      const { row, col } = parseKey(key)
      ctx.strokeStyle = "#0e639c"
      ctx.lineWidth = 3
      ctx.strokeRect(col * cellWidth, row * cellHeight, cellWidth, cellHeight)
    })
  }
```

**Step 2: Add canvas element to UI**

Add after control panel, before closing `</div>`:

```svelte
  {#if spriteImage}
    <div class="canvas-container">
      <canvas
        bind:this={canvas}
        width="800"
        height="600"
        class:multi-mode={selectionMode === "multi"}
      >
      </canvas>
    </div>
  {:else}
    <div class="empty-state">
      <p>이미지를 불러와서 시작하세요</p>
    </div>
  {/if}
```

**Step 3: Add canvas styles**

Add to `<style>` section:

```css
  .canvas-container {
    margin-bottom: 2rem;
  }

  canvas {
    width: 100%;
    max-width: 800px;
    height: auto;
    background: #1a1a1a;
    border: 2px solid #333;
    border-radius: 8px;
    cursor: pointer;
  }

  canvas.multi-mode {
    cursor: crosshair;
  }

  .empty-state {
    padding: 4rem 2rem;
    text-align: center;
    background: #1a1a1a;
    border: 2px dashed #333;
    border-radius: 8px;
    margin-bottom: 2rem;
  }

  .empty-state p {
    margin: 0;
    color: #666;
    font-size: 1.1rem;
  }
```

**Step 4: Test canvas rendering**

Run: `bun run dev`
Expected: Load an image, see it displayed with grid overlay, change grid size to see grid update

**Step 5: Commit**

```bash
git add src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte
git commit -m "feat: implement canvas grid rendering

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 4: Implement single-cell selection and modal

**Files:**
- Modify: `src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte`

**Step 1: Add canvas click handler**

Add after `drawGrid` function:

```typescript
  let lastSelectedCell = $state<CellKey | null>(null)

  function handleCanvasClick(e: MouseEvent) {
    if (!canvas || !spriteImage) return

    const rect = canvas.getBoundingClientRect()
    const scaleX = canvas.width / rect.width
    const scaleY = canvas.height / rect.height
    const x = (e.clientX - rect.left) * scaleX
    const y = (e.clientY - rect.top) * scaleY

    const cellWidth = canvas.width / gridCols
    const cellHeight = canvas.height / gridRows

    const col = Math.floor(x / cellWidth)
    const row = Math.floor(y / cellHeight)

    if (row < 0 || row >= gridRows || col < 0 || col >= gridCols) return

    const key = cellKey(row, col)

    if (selectionMode === "single") {
      currentCell = { row, col }
      showModal = true
    } else {
      // Multi mode - will implement in next task
      if (e.ctrlKey || e.metaKey) {
        if (selectedCells.has(key)) {
          selectedCells.delete(key)
        } else {
          selectedCells.add(key)
        }
        selectedCells = new Set(selectedCells)
      }
      lastSelectedCell = { row, col }
      drawGrid()
    }
  }
```

**Step 2: Add modal state and functions**

Add after click handler:

```typescript
  let modalDescription = $state("")

  function openModal() {
    if (!currentCell) return
    const key = cellKey(currentCell.row, currentCell.col)
    modalDescription = cellDescriptions.get(key) || ""
  }

  function saveDescription() {
    if (!currentCell) return
    const key = cellKey(currentCell.row, currentCell.col)

    if (modalDescription.trim()) {
      cellDescriptions.set(key, modalDescription.trim())
    } else {
      cellDescriptions.delete(key)
    }

    cellDescriptions = new Map(cellDescriptions)
    showModal = false
    modalDescription = ""
    drawGrid()
  }

  function deleteDescription() {
    if (!currentCell) return
    const key = cellKey(currentCell.row, currentCell.col)
    cellDescriptions.delete(key)
    cellDescriptions = new Map(cellDescriptions)
    showModal = false
    modalDescription = ""
    drawGrid()
  }

  function closeModal() {
    showModal = false
    modalDescription = ""
    currentCell = null
  }

  $effect(() => {
    if (showModal && currentCell) {
      openModal()
    }
  })
```

**Step 3: Add modal UI**

Add before closing `</div>`, after canvas:

```svelte
  {#if showModal && currentCell}
    <div class="modal-backdrop" onclick={closeModal}>
      <div class="modal" onclick={(e) => e.stopPropagation()}>
        <div class="modal-header">
          <h3>
            셀 설명 편집
            <span class="cell-info">
              (행: {currentCell.row}, 열: {currentCell.col},
              인덱스: {cellToIndex(currentCell.row, currentCell.col)})
            </span>
          </h3>
          <button class="close-btn" onclick={closeModal}>×</button>
        </div>

        <div class="modal-body">
          <textarea
            bind:value={modalDescription}
            placeholder="셀 설명을 입력하세요..."
            rows="4"
          ></textarea>
        </div>

        <div class="modal-footer">
          <button class="btn-secondary" onclick={closeModal}>취소</button>
          <button class="btn-danger" onclick={deleteDescription}>삭제</button>
          <button class="btn-primary" onclick={saveDescription}>저장</button>
        </div>
      </div>
    </div>
  {/if}
```

**Step 4: Update canvas element**

Update canvas element to include click handler:

```svelte
      <canvas
        bind:this={canvas}
        width="800"
        height="600"
        class:multi-mode={selectionMode === "multi"}
        onclick={handleCanvasClick}
      >
      </canvas>
```

**Step 5: Add modal styles**

Add to `<style>` section:

```css
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background: #1e1e1e;
    border: 1px solid #444;
    border-radius: 8px;
    width: 90%;
    max-width: 500px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 1.5rem;
    border-bottom: 1px solid #333;
  }

  .modal-header h3 {
    margin: 0;
    font-size: 1rem;
    color: #0e639c;
  }

  .cell-info {
    display: block;
    font-size: 0.85rem;
    color: #888;
    font-weight: normal;
    margin-top: 0.25rem;
  }

  .close-btn {
    background: none;
    border: none;
    color: #888;
    font-size: 1.5rem;
    cursor: pointer;
    padding: 0;
    width: 30px;
    height: 30px;
    line-height: 1;
  }

  .close-btn:hover {
    color: #fff;
  }

  .modal-body {
    padding: 1.5rem;
  }

  .modal-body textarea {
    width: 100%;
    padding: 0.75rem;
    background: #0d0d0d;
    border: 1px solid #444;
    color: #e0e0e0;
    border-radius: 4px;
    font-family: inherit;
    font-size: 0.9rem;
    resize: vertical;
  }

  .modal-body textarea:focus {
    outline: none;
    border-color: #0e639c;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    padding: 1.5rem;
    border-top: 1px solid #333;
  }

  .modal-footer button {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
  }

  .btn-primary {
    background: #0e639c;
    color: #fff;
  }

  .btn-primary:hover {
    background: #1177bb;
  }

  .btn-secondary {
    background: #333;
    color: #ccc;
  }

  .btn-secondary:hover {
    background: #444;
  }

  .btn-danger {
    background: #d32f2f;
    color: #fff;
  }

  .btn-danger:hover {
    background: #f44336;
  }
```

**Step 6: Test modal functionality**

Run: `bun run dev`
Expected: Load image, single mode, click cell → modal opens, enter text, save → cell highlighted, click again → text preserved

**Step 7: Commit**

```bash
git add src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte
git commit -m "feat: implement single-cell selection and modal

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 5: Implement multi-cell selection with Shift

**Files:**
- Modify: `src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte`

**Step 1: Add rectangle selection function**

Add after `handleCanvasClick`:

```typescript
  function selectRectangle(start: CellKey | null, end: CellKey) {
    if (!start) {
      const key = cellKey(end.row, end.col)
      selectedCells.add(key)
      selectedCells = new Set(selectedCells)
      return
    }

    const minRow = Math.min(start.row, end.row)
    const maxRow = Math.max(start.row, end.row)
    const minCol = Math.min(start.col, end.col)
    const maxCol = Math.max(start.col, end.col)

    for (let r = minRow; r <= maxRow; r++) {
      for (let c = minCol; c <= maxCol; c++) {
        selectedCells.add(cellKey(r, c))
      }
    }

    selectedCells = new Set(selectedCells)
  }
```

**Step 2: Update handleCanvasClick for Shift selection**

Replace the multi mode section in `handleCanvasClick`:

```typescript
    if (selectionMode === "single") {
      currentCell = { row, col }
      showModal = true
    } else {
      // Multi mode
      if (e.shiftKey && lastSelectedCell) {
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
      } else {
        // Single click in multi mode - clear and select one
        selectedCells.clear()
        selectedCells.add(key)
        selectedCells = new Set(selectedCells)
      }
      lastSelectedCell = { row, col }
      drawGrid()
    }
```

**Step 3: Add clear selection button**

Update the selection mode section in control panel:

```svelte
    <div class="control-section">
      <h3>선택 모드</h3>
      <div class="mode-toggle">
        <button
          class:active={selectionMode === "single"}
          onclick={() => { selectionMode = "single"; selectedCells.clear() }}
        >
          단일 선택
        </button>
        <button
          class:active={selectionMode === "multi"}
          onclick={() => selectionMode = "multi"}
        >
          멀티 선택
        </button>
      </div>
      {#if selectionMode === "multi"}
        <div class="multi-info">
          {#if selectedCells.size > 0}
            <span class="selection-count">{selectedCells.size}개 셀 선택됨</span>
            <button class="clear-btn" onclick={() => { selectedCells.clear(); drawGrid() }}>
              선택 해제
            </button>
          {:else}
            <span class="hint">클릭: 단일 선택 | Ctrl+클릭: 추가/제거 | Shift+클릭: 영역 선택</span>
          {/if}
        </div>
      {/if}
    </div>
```

**Step 4: Add multi-info styles**

Add to `<style>` section:

```css
  .multi-info {
    margin-top: 0.75rem;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .hint {
    font-size: 0.8rem;
    color: #666;
  }

  .clear-btn {
    padding: 0.25rem 0.75rem;
    background: #d32f2f;
    border: none;
    color: #fff;
    cursor: pointer;
    border-radius: 4px;
    font-size: 0.8rem;
  }

  .clear-btn:hover {
    background: #f44336;
  }
```

**Step 5: Test multi-selection**

Run: `bun run dev`
Expected: Switch to multi mode, click cell → selected, Ctrl+click another → both selected, Shift+click → rectangle selected, clear button works

**Step 6: Commit**

```bash
git add src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte
git commit -m "feat: implement multi-cell selection with Shift+Ctrl

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 6: Implement preset tags

**Files:**
- Modify: `src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte`

**Step 1: Define preset tags**

Add after state declarations:

```typescript
  const PRESET_TAGS = [
    // 지형
    "벽(위)", "벽(아래)", "벽(좌)", "벽(우)", "바닥", "천장",
    // 구조물
    "문", "창문", "계단", "사다리",
    // 자연
    "풀", "나무", "물", "돌",
    // 상호작용
    "상자", "스위치", "레버",
    // 캐릭터
    "idle", "walk", "run", "jump", "attack"
  ]
```

**Step 2: Add tag application function**

Add after preset tags:

```typescript
  function applyTag(tag: string) {
    if (selectionMode === "single" && currentCell) {
      const key = cellKey(currentCell.row, currentCell.col)
      cellDescriptions.set(key, tag)
      cellDescriptions = new Map(cellDescriptions)
      drawGrid()
    } else if (selectionMode === "multi" && selectedCells.size > 0) {
      selectedCells.forEach(key => {
        cellDescriptions.set(key, tag)
      })
      cellDescriptions = new Map(cellDescriptions)
      selectedCells.clear()
      drawGrid()
    }
  }
```

**Step 3: Add preset tags to modal**

Update modal body to include quick tags section:

```svelte
        <div class="modal-body">
          <textarea
            bind:value={modalDescription}
            placeholder="셀 설명을 입력하세요..."
            rows="4"
          ></textarea>

          <div class="quick-tags">
            <span class="tags-label">빠른 선택:</span>
            <div class="tags-grid">
              {#each PRESET_TAGS as tag}
                <button
                  class="tag-btn preset-tag"
                  onclick={() => { modalDescription = tag }}
                >
                  {tag}
                </button>
              {/each}
            </div>
          </div>
        </div>
```

**Step 4: Add preset tags toolbar below canvas**

Add after canvas container, before output section:

```svelte
  {#if spriteImage}
    <div class="tags-toolbar">
      <h3>프리셋 태그</h3>
      <div class="tags-toolbar-content">
        {#each PRESET_TAGS as tag}
          <button
            class="tag-btn preset-tag"
            onclick={() => applyTag(tag)}
            disabled={selectionMode === "single" || selectedCells.size === 0}
          >
            {tag}
          </button>
        {/each}
      </div>
      {#if selectionMode === "multi" && selectedCells.size === 0}
        <p class="toolbar-hint">멀티 선택 모드에서 셀을 선택한 후 태그를 클릭하세요</p>
      {/if}
    </div>
  {/if}
```

**Step 5: Add tag styles**

Add to `<style>` section:

```css
  .quick-tags {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid #333;
  }

  .tags-label {
    display: block;
    font-size: 0.85rem;
    color: #888;
    margin-bottom: 0.5rem;
  }

  .tags-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .tag-btn {
    padding: 0.4rem 0.8rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
    transition: all 0.2s;
  }

  .preset-tag {
    background: #0e639c;
    color: #fff;
  }

  .preset-tag:hover:not(:disabled) {
    background: #1177bb;
    transform: translateY(-1px);
  }

  .tag-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .tags-toolbar {
    background: #1e1e1e;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 1.5rem;
    margin-bottom: 2rem;
  }

  .tags-toolbar h3 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
    color: #0e639c;
  }

  .tags-toolbar-content {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .toolbar-hint {
    margin: 1rem 0 0 0;
    font-size: 0.85rem;
    color: #666;
  }
```

**Step 6: Test preset tags**

Run: `bun run dev`
Expected: In modal, click preset tag → fills textarea. In multi mode, select cells → click toolbar tag → all selected cells get description

**Step 7: Commit**

```bash
git add src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte
git commit -m "feat: implement preset tags for quick input

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 7: Implement custom tags with localStorage

**Files:**
- Modify: `src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte`

**Step 1: Add localStorage functions**

Add after preset tags definition:

```typescript
  const CUSTOM_TAGS_KEY = "sprite-sheet-describer-custom-tags"

  function loadCustomTags() {
    const stored = localStorage.getItem(CUSTOM_TAGS_KEY)
    if (stored) {
      try {
        customTags = JSON.parse(stored)
      } catch {
        customTags = []
      }
    }
  }

  function saveCustomTags() {
    localStorage.setItem(CUSTOM_TAGS_KEY, JSON.stringify(customTags))
  }

  // Load on mount
  $effect(() => {
    loadCustomTags()
  })
```

**Step 2: Add custom tag management state and functions**

Add after localStorage functions:

```typescript
  let showAddTagModal = $state(false)
  let newTagName = $state("")

  function addCustomTag() {
    const tag = newTagName.trim()
    if (tag && !customTags.includes(tag) && !PRESET_TAGS.includes(tag)) {
      customTags = [...customTags, tag]
      saveCustomTags()
      newTagName = ""
      showAddTagModal = false
    }
  }

  function removeCustomTag(tag: string) {
    customTags = customTags.filter(t => t !== tag)
    saveCustomTags()
  }
```

**Step 3: Add custom tags to toolbar**

Update tags toolbar to include custom tags:

```svelte
  {#if spriteImage}
    <div class="tags-toolbar">
      <div class="toolbar-header">
        <h3>프리셋 태그</h3>
        <button class="add-tag-btn" onclick={() => showAddTagModal = true}>
          + 커스텀 태그 추가
        </button>
      </div>

      <div class="tags-toolbar-content">
        {#each PRESET_TAGS as tag}
          <button
            class="tag-btn preset-tag"
            onclick={() => applyTag(tag)}
            disabled={selectionMode === "single" || selectedCells.size === 0}
          >
            {tag}
          </button>
        {/each}
      </div>

      {#if customTags.length > 0}
        <div class="custom-tags-section">
          <h4>커스텀 태그</h4>
          <div class="tags-toolbar-content">
            {#each customTags as tag}
              <button
                class="tag-btn custom-tag"
                onclick={() => applyTag(tag)}
                disabled={selectionMode === "single" || selectedCells.size === 0}
              >
                {tag}
                <span class="remove-tag" onclick={(e) => { e.stopPropagation(); removeCustomTag(tag) }}>×</span>
              </button>
            {/each}
          </div>
        </div>
      {/if}

      {#if selectionMode === "multi" && selectedCells.size === 0}
        <p class="toolbar-hint">멀티 선택 모드에서 셀을 선택한 후 태그를 클릭하세요</p>
      {/if}
    </div>
  {/if}
```

**Step 4: Add custom tag modal**

Add before closing `</div>`:

```svelte
  {#if showAddTagModal}
    <div class="modal-backdrop" onclick={() => showAddTagModal = false}>
      <div class="modal small-modal" onclick={(e) => e.stopPropagation()}>
        <div class="modal-header">
          <h3>커스텀 태그 추가</h3>
          <button class="close-btn" onclick={() => showAddTagModal = false}>×</button>
        </div>

        <div class="modal-body">
          <input
            type="text"
            bind:value={newTagName}
            placeholder="태그 이름을 입력하세요..."
            onkeydown={(e) => e.key === "Enter" && addCustomTag()}
          />
        </div>

        <div class="modal-footer">
          <button class="btn-secondary" onclick={() => showAddTagModal = false}>취소</button>
          <button class="btn-primary" onclick={addCustomTag}>추가</button>
        </div>
      </div>
    </div>
  {/if}
```

**Step 5: Update modal quick tags to include custom tags**

Update the quick tags section in the cell edit modal:

```svelte
          <div class="quick-tags">
            <span class="tags-label">빠른 선택:</span>
            <div class="tags-grid">
              {#each PRESET_TAGS as tag}
                <button
                  class="tag-btn preset-tag"
                  onclick={() => { modalDescription = tag }}
                >
                  {tag}
                </button>
              {/each}
              {#each customTags as tag}
                <button
                  class="tag-btn custom-tag"
                  onclick={() => { modalDescription = tag }}
                >
                  {tag}
                </button>
              {/each}
            </div>
          </div>
```

**Step 6: Add custom tag styles**

Add to `<style>` section:

```css
  .toolbar-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .add-tag-btn {
    padding: 0.4rem 0.8rem;
    background: #4CAF50;
    border: none;
    color: #fff;
    cursor: pointer;
    border-radius: 4px;
    font-size: 0.85rem;
  }

  .add-tag-btn:hover {
    background: #66BB6A;
  }

  .custom-tags-section {
    margin-top: 1.5rem;
    padding-top: 1.5rem;
    border-top: 1px solid #333;
  }

  .custom-tags-section h4 {
    margin: 0 0 1rem 0;
    font-size: 0.9rem;
    color: #4CAF50;
  }

  .custom-tag {
    background: #4CAF50;
    color: #fff;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .custom-tag:hover:not(:disabled) {
    background: #66BB6A;
  }

  .remove-tag {
    font-size: 1.2rem;
    line-height: 1;
    opacity: 0.7;
  }

  .remove-tag:hover {
    opacity: 1;
  }

  .small-modal {
    max-width: 400px;
  }

  .modal-body input[type="text"] {
    width: 100%;
    padding: 0.75rem;
    background: #0d0d0d;
    border: 1px solid #444;
    color: #e0e0e0;
    border-radius: 4px;
    font-size: 0.9rem;
  }

  .modal-body input[type="text"]:focus {
    outline: none;
    border-color: #4CAF50;
  }
```

**Step 7: Test custom tags**

Run: `bun run dev`
Expected: Click "커스텀 태그 추가", enter name, see it appear in toolbar with × button, click × to remove, reload page → custom tags persist

**Step 8: Commit**

```bash
git add src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte
git commit -m "feat: implement custom tags with localStorage

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 8: Implement output formats (JSON and text)

**Files:**
- Modify: `src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte`

**Step 1: Add output generation functions**

Add after custom tag functions:

```typescript
  function generateJSON(): string {
    const cells = Array.from(cellDescriptions.entries()).map(([key, description]) => {
      const { row, col } = parseKey(key)

      if (outputFormat === "index") {
        return {
          index: cellToIndex(row, col),
          description
        }
      } else {
        return {
          row,
          col,
          description
        }
      }
    })

    const output = {
      gridSize: { rows: gridRows, cols: gridCols },
      format: outputFormat,
      cells
    }

    return JSON.stringify(output, null, 2)
  }

  function generateTextList(): string {
    const lines = Array.from(cellDescriptions.entries())
      .map(([key, description]) => {
        const { row, col } = parseKey(key)

        if (outputFormat === "index") {
          const index = cellToIndex(row, col)
          return `[${index}] ${description}`
        } else {
          return `[${row},${col}] ${description}`
        }
      })
      .sort()

    return lines.join("\n")
  }

  let jsonOutput = $derived(cellDescriptions.size > 0 ? generateJSON() : "")
  let textOutput = $derived(cellDescriptions.size > 0 ? generateTextList() : "")
```

**Step 2: Add copy to clipboard function**

Add after output generation:

```typescript
  let copyFeedback = $state<string | null>(null)

  async function copyToClipboard(text: string, label: string) {
    try {
      await navigator.clipboard.writeText(text)
      copyFeedback = label
      setTimeout(() => { copyFeedback = null }, 2000)
    } catch (err) {
      console.error("Failed to copy:", err)
    }
  }
```

**Step 3: Add output section UI**

Add before closing `</div>`, after custom tag modal:

```svelte
  {#if cellDescriptions.size > 0}
    <div class="output-section">
      <div class="output-header">
        <h3>출력</h3>
        <div class="format-toggle">
          <label>
            <input
              type="radio"
              value="rowcol"
              bind:group={outputFormat}
            />
            Row/Col
          </label>
          <label>
            <input
              type="radio"
              value="index"
              bind:group={outputFormat}
            />
            Index
          </label>
        </div>
      </div>

      <div class="output-panels">
        <div class="output-panel">
          <div class="panel-header">
            <h4>JSON</h4>
            <button onclick={() => copyToClipboard(jsonOutput, "JSON")}>
              {copyFeedback === "JSON" ? "복사됨!" : "Copy"}
            </button>
          </div>
          <pre>{jsonOutput}</pre>
        </div>

        <div class="output-panel">
          <div class="panel-header">
            <h4>텍스트 리스트</h4>
            <button onclick={() => copyToClipboard(textOutput, "텍스트")}>
              {copyFeedback === "텍스트" ? "복사됨!" : "Copy"}
            </button>
          </div>
          <pre>{textOutput}</pre>
        </div>
      </div>
    </div>
  {/if}
```

**Step 4: Add output styles**

Add to `<style>` section:

```css
  .output-section {
    margin-top: 2rem;
  }

  .output-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .output-header h3 {
    margin: 0;
    font-size: 1.2rem;
    color: #0e639c;
  }

  .format-toggle {
    display: flex;
    gap: 1rem;
  }

  .format-toggle label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.9rem;
    color: #ccc;
    cursor: pointer;
  }

  .format-toggle input[type="radio"] {
    cursor: pointer;
    accent-color: #0e639c;
  }

  .output-panels {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  .output-panel {
    background: #1e1e1e;
    border: 1px solid #333;
    border-radius: 8px;
    overflow: hidden;
  }

  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background: #252526;
    border-bottom: 1px solid #333;
  }

  .panel-header h4 {
    margin: 0;
    font-size: 0.9rem;
    color: #0e639c;
  }

  .panel-header button {
    padding: 0.4rem 0.8rem;
    background: #0e639c;
    border: none;
    color: #fff;
    cursor: pointer;
    border-radius: 4px;
    font-size: 0.85rem;
  }

  .panel-header button:hover {
    background: #1177bb;
  }

  .output-panel pre {
    margin: 0;
    padding: 1rem;
    background: #0d0d0d;
    color: #e0e0e0;
    font-family: "Monaco", "Menlo", "Ubuntu Mono", monospace;
    font-size: 0.85rem;
    white-space: pre-wrap;
    word-wrap: break-word;
    overflow-x: auto;
    max-height: 400px;
    overflow-y: auto;
  }
```

**Step 5: Test output formats**

Run: `bun run dev`
Expected: Add descriptions to cells → see JSON and text output, toggle Row/Col vs Index → see format change, click Copy → clipboard has content, see "복사됨!" feedback

**Step 6: Commit**

```bash
git add src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte
git commit -m "feat: implement JSON and text output formats

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 9: Implement project save/load

**Files:**
- Modify: `src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte`

**Step 1: Add project save/load functions**

Add after output functions:

```typescript
  interface ProjectData {
    imageName: string
    imageDataUrl: string
    gridSize: { rows: number, cols: number }
    descriptions: Record<string, string>
    customTags: string[]
  }

  async function saveProject() {
    if (!spriteImage || !imageFile) return

    const descriptions: Record<string, string> = {}
    cellDescriptions.forEach((value, key) => {
      descriptions[key] = value
    })

    // Convert image to data URL
    const canvas = document.createElement("canvas")
    canvas.width = spriteImage.width
    canvas.height = spriteImage.height
    const ctx = canvas.getContext("2d")
    if (!ctx) return
    ctx.drawImage(spriteImage, 0, 0)
    const imageDataUrl = canvas.toDataURL("image/png")

    const projectData: ProjectData = {
      imageName: imageFile.name,
      imageDataUrl,
      gridSize: { rows: gridRows, cols: gridCols },
      descriptions,
      customTags
    }

    const blob = new Blob([JSON.stringify(projectData, null, 2)], {
      type: "application/json"
    })

    const url = URL.createObjectURL(blob)
    const a = document.createElement("a")
    a.href = url
    a.download = `sprite-sheet-project-${Date.now()}.json`
    a.click()
    URL.revokeObjectURL(url)
  }

  function handleProjectLoad(e: Event) {
    const input = e.target as HTMLInputElement
    const file = input.files?.[0]
    if (!file) return

    const reader = new FileReader()

    reader.onload = (evt) => {
      try {
        const projectData: ProjectData = JSON.parse(evt.target?.result as string)

        // Load image
        const img = new Image()
        img.onload = () => {
          spriteImage = img
          imageFile = new File([projectData.imageDataUrl], projectData.imageName, { type: "image/png" })
          gridRows = projectData.gridSize.rows
          gridCols = projectData.gridSize.cols

          // Load descriptions
          cellDescriptions.clear()
          Object.entries(projectData.descriptions).forEach(([key, value]) => {
            cellDescriptions.set(key, value)
          })
          cellDescriptions = new Map(cellDescriptions)

          // Load custom tags
          customTags = projectData.customTags || []
          saveCustomTags()

          drawGrid()
        }
        img.src = projectData.imageDataUrl
      } catch (err) {
        console.error("Failed to load project:", err)
        alert("프로젝트 파일을 불러올 수 없습니다.")
      }
    }

    reader.readAsText(file)
  }
```

**Step 2: Add reset functions**

Add after save/load functions:

```typescript
  function resetAll() {
    if (!confirm("모든 데이터를 초기화하시겠습니까?")) return

    spriteImage = null
    imageFile = null
    cellDescriptions.clear()
    selectedCells.clear()
    gridRows = 4
    gridCols = 4
    if (canvas) {
      const ctx = canvas.getContext("2d")
      if (ctx) {
        ctx.clearRect(0, 0, canvas.width, canvas.height)
      }
    }
  }

  function clearDescriptions() {
    if (!confirm("모든 설명을 삭제하시겠습니까?")) return

    cellDescriptions.clear()
    selectedCells.clear()
    drawGrid()
  }
```

**Step 3: Add project management section**

Add after output section, before closing `</div>`:

```svelte
  <div class="project-management">
    <h3>프로젝트 관리</h3>
    <div class="management-actions">
      <button class="mgmt-btn save" onclick={saveProject} disabled={!spriteImage}>
        💾 프로젝트 저장
      </button>
      <label class="mgmt-btn load">
        📂 프로젝트 불러오기
        <input
          type="file"
          accept="application/json"
          onchange={handleProjectLoad}
          style="display: none;"
        />
      </label>
      <button class="mgmt-btn clear" onclick={clearDescriptions} disabled={cellDescriptions.size === 0}>
        🗑️ 설명만 지우기
      </button>
      <button class="mgmt-btn reset" onclick={resetAll} disabled={!spriteImage}>
        ♻️ 전체 초기화
      </button>
    </div>
  </div>
```

**Step 4: Add project management styles**

Add to `<style>` section:

```css
  .project-management {
    margin-top: 2rem;
    padding-top: 2rem;
    border-top: 2px solid #333;
  }

  .project-management h3 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
    color: #888;
  }

  .management-actions {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .mgmt-btn {
    padding: 0.75rem 1.25rem;
    border: 1px solid #444;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.9rem;
    transition: all 0.2s;
  }

  .mgmt-btn.save {
    background: #0e639c;
    color: #fff;
  }

  .mgmt-btn.save:hover:not(:disabled) {
    background: #1177bb;
  }

  .mgmt-btn.load {
    background: #4CAF50;
    color: #fff;
  }

  .mgmt-btn.load:hover {
    background: #66BB6A;
  }

  .mgmt-btn.clear {
    background: #ff9800;
    color: #fff;
  }

  .mgmt-btn.clear:hover:not(:disabled) {
    background: #ffa726;
  }

  .mgmt-btn.reset {
    background: #d32f2f;
    color: #fff;
  }

  .mgmt-btn.reset:hover:not(:disabled) {
    background: #f44336;
  }

  .mgmt-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
```

**Step 5: Test project save/load**

Run: `bun run dev`
Expected: Add descriptions → save project → reload page → load project → all data restored including image, grid, descriptions, custom tags

**Step 6: Commit**

```bash
git add src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte
git commit -m "feat: implement project save/load and reset functions

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 10: Add error handling and validation

**Files:**
- Modify: `src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte`

**Step 1: Add error state and validation**

Add to state section:

```typescript
  let errorMessage = $state<string | null>(null)

  function showError(message: string) {
    errorMessage = message
    setTimeout(() => { errorMessage = null }, 4000)
  }

  function validateGridSize(value: number, name: string) {
    if (value < 1) {
      showError(`${name}은(는) 1 이상이어야 합니다.`)
      return 1
    }
    if (value > 50) {
      showError(`${name}은(는) 50 이하여야 합니다.`)
      return 50
    }
    return value
  }
```

**Step 2: Update grid input validation**

Update grid inputs to use validation:

```svelte
        <label>
          행:
          <input
            type="number"
            min="1"
            max="50"
            bind:value={gridRows}
            onchange={() => {
              gridRows = validateGridSize(gridRows, "행")
              drawGrid()
            }}
          />
        </label>
        <label>
          열:
          <input
            type="number"
            min="1"
            max="50"
            bind:value={gridCols}
            onchange={() => {
              gridCols = validateGridSize(gridCols, "열")
              drawGrid()
            }}
          />
        </label>
```

**Step 3: Add error handling to image loading**

Update `handleFileSelect`:

```typescript
  function handleFileSelect(e: Event) {
    const input = e.target as HTMLInputElement
    const file = input.files?.[0]
    if (!file) return

    if (!file.type.startsWith("image/")) {
      showError("이미지 파일만 선택할 수 있습니다.")
      return
    }

    if (file.size > 10 * 1024 * 1024) {
      showError("파일 크기는 10MB 이하여야 합니다.")
      return
    }

    imageFile = file
    const reader = new FileReader()

    reader.onerror = () => {
      showError("이미지를 불러올 수 없습니다.")
    }

    reader.onload = (evt) => {
      const img = new Image()

      img.onerror = () => {
        showError("이미지를 로드할 수 없습니다.")
      }

      img.onload = () => {
        spriteImage = img
        cellDescriptions.clear()
        selectedCells.clear()
        drawGrid()
      }

      img.src = evt.target?.result as string
    }

    reader.readAsDataURL(file)
  }
```

**Step 4: Add error message UI**

Add after tool header:

```svelte
  {#if errorMessage}
    <div class="error-toast">
      ⚠️ {errorMessage}
    </div>
  {/if}
```

**Step 5: Add error toast styles**

Add to `<style>` section:

```css
  .error-toast {
    position: fixed;
    top: 2rem;
    right: 2rem;
    background: #d32f2f;
    color: #fff;
    padding: 1rem 1.5rem;
    border-radius: 6px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    z-index: 2000;
    animation: slideIn 0.3s ease-out;
  }

  @keyframes slideIn {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }
```

**Step 6: Test error handling**

Run: `bun run dev`
Expected: Try to load non-image file → error. Try to set grid to 0 → corrected to 1 with error. Try to set grid to 100 → corrected to 50 with error.

**Step 7: Commit**

```bash
git add src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte
git commit -m "feat: add error handling and validation

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 11: Final testing and documentation

**Files:**
- Modify: `CLAUDE.md`

**Step 1: Full integration test**

Run: `bun run dev`

Test checklist:
- [ ] Load sprite sheet image
- [ ] Adjust grid size (rows/cols)
- [ ] Single mode: click cell → modal opens → add description
- [ ] Multi mode: click cell → selected
- [ ] Multi mode: Ctrl+click → toggle selection
- [ ] Multi mode: Shift+click → rectangle selection
- [ ] Apply preset tag to multiple cells
- [ ] Add custom tag → appears in toolbar
- [ ] Remove custom tag
- [ ] Toggle output format (Row/Col vs Index)
- [ ] Copy JSON output
- [ ] Copy text list output
- [ ] Save project → download JSON
- [ ] Load project → all data restored
- [ ] Clear descriptions only
- [ ] Reset all

**Step 2: Update project documentation**

Edit `CLAUDE.md` to add new tool info:

```markdown
## Project Structure

```
/src                          # Frontend (SvelteKit)
├── routes/
│   ├── tools/
│   │   ├── encoding/
│   │   │   └── base64/         # Base64 encoder/decoder
│   │   └── llm-toolset/
│   │       ├── motion-descriptor/      # Mouse motion to LLM prompt
│   │       └── sprite-sheet-describer/ # Sprite sheet grid annotator
```

## Tools

### Sprite Sheet Describer

스프라이트 시트를 n×n 그리드로 나누고 각 셀에 설명을 추가하는 도구입니다.

**Features:**
- 이미지 파일 로드 (drag & drop or file picker)
- 사용자 지정 그리드 크기 (1×1 ~ 50×50)
- 단일/멀티 셀 선택 모드
- 프리셋 태그 (지형, 구조물, 캐릭터 등)
- 커스텀 태그 (localStorage 저장)
- 출력 형식: JSON, 텍스트 리스트
- 인덱스 형식: Row/Col 또는 Index
- 프로젝트 저장/불러오기

**Usage:**
1. 이미지 불러오기
2. 그리드 크기 설정
3. 셀 클릭하여 설명 입력 또는 태그 사용
4. 출력 복사
```

**Step 3: Verify build**

Run: `bun run check`
Expected: No TypeScript errors

Run: `cargo fmt && cargo clippy` in `src-tauri/`
Expected: No warnings (if any Rust changes were made, but there shouldn't be any)

**Step 4: Final commit**

```bash
git add CLAUDE.md
git commit -m "docs: add sprite sheet describer to project docs

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Plan Complete

**Implementation Summary:**
- Created new tool route: `/tools/llm-toolset/sprite-sheet-describer`
- Canvas-based grid rendering system
- Single and multi-cell selection modes
- Preset tags + custom tags (localStorage)
- JSON and text list output with format toggle
- Project save/load functionality
- Error handling and validation

**Total Files Modified:**
- `src/lib/config/tools.ts` - Added tool to navigation
- `src/routes/tools/llm-toolset/sprite-sheet-describer/+page.svelte` - Main implementation
- `CLAUDE.md` - Documentation

**No Rust backend needed** - Pure frontend implementation using Canvas API and browser APIs.

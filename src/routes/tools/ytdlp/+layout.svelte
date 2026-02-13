<script lang="ts">
  import { commands } from "$lib/bindings"
  import { Channel } from "@tauri-apps/api/core"
  import { listen } from "@tauri-apps/api/event"
  import { page } from "$app/stores"
  import { onMount, onDestroy } from "svelte"

  let { children } = $props()

  let checking = $state(true)
  let depsInstalled = $state(false)
  let installing = $state(false)
  let installMessage = $state("")
  let installError = $state("")

  // Responsive sidebar
  let sidebarCollapsed = $state(false)
  let windowWidth = $state(1024)

  // Drawer state
  let drawerOpen = $state(false)
  let activeDownloads = $state<any[]>([])
  let recentCompleted = $state<any[]>([])
  let activeCount = $derived(activeDownloads.filter(d => d.status === "downloading").length)
  let pendingCount = $derived(activeDownloads.filter(d => d.status === "pending").length)

  const navItems = [
    { href: "/tools/ytdlp", label: "Home", icon: "home" },
    { href: "/tools/ytdlp/queue", label: "Downloads", icon: "download_done" },
    { href: "/tools/ytdlp/history", label: "Library", icon: "library_books" },
    { href: "/tools/ytdlp/settings", label: "Settings", icon: "settings" },
  ]

  function isActive(href: string): boolean {
    const path = $page.url.pathname
    if (href === "/tools/ytdlp") return path === "/tools/ytdlp"
    return path.startsWith(href)
  }

  let isSmall = $derived(windowWidth < 768)
  let collapsed = $derived(sidebarCollapsed || isSmall)

  // Drawer auto-refresh
  let drawerInterval: ReturnType<typeof setInterval> | null = null
  let unlisten: (() => void) | null = null

  async function loadActiveDownloads() {
    try {
      const result = await commands.getActiveDownloads()
      if (result.status === "ok") {
        activeDownloads = result.data
      }
    } catch {}
  }

  async function loadRecentCompleted() {
    try {
      const result = await commands.getDownloadQueue()
      if (result.status === "ok") {
        recentCompleted = result.data.filter((d: any) => d.status === "completed").slice(0, 5)
      }
    } catch {}
  }

  function startDrawerRefresh() {
    loadActiveDownloads()
    loadRecentCompleted()
    drawerInterval = setInterval(() => {
      loadActiveDownloads()
      loadRecentCompleted()
    }, 2000)
  }

  function stopDrawerRefresh() {
    if (drawerInterval) {
      clearInterval(drawerInterval)
      drawerInterval = null
    }
  }

  $effect(() => {
    if (drawerOpen) {
      startDrawerRefresh()
    } else {
      stopDrawerRefresh()
    }
  })

  onMount(async () => {
    windowWidth = window.innerWidth
    await checkDeps()

    // Listen for global download events to update badge count
    try {
      const unlistenFn = await listen("download-event", () => {
        loadActiveDownloads()
      })
      unlisten = unlistenFn
    } catch {}

    // Initial load for badge
    loadActiveDownloads()
  })

  onDestroy(() => {
    stopDrawerRefresh()
    if (unlisten) unlisten()
  })

  function handleResize() {
    windowWidth = window.innerWidth
  }

  async function checkDeps() {
    checking = true
    try {
      const result = await commands.checkDependencies()
      if (result.status === "ok") {
        depsInstalled = result.data.ytdlpInstalled
      }
    } catch (e) {
      console.error(e)
    } finally {
      checking = false
    }
  }

  async function handleInstall() {
    installing = true
    installError = ""
    installMessage = "yt-dlp 다운로드 중..."

    try {
      const channel = new Channel()
      channel.onmessage = (event: any) => {
        if (event.event === "progress") {
          installMessage = event.data.message
        } else if (event.event === "completed") {
          installMessage = event.data.message
        } else if (event.event === "error") {
          installError = event.data.message
        }
      }

      const result = await commands.installDependencies(channel)
      if (result.status === "error") {
        installError = JSON.stringify(result.error)
      } else {
        await checkDeps()
      }
    } catch (e: any) {
      installError = e.message || String(e)
    } finally {
      installing = false
    }
  }
</script>

<svelte:window onresize={handleResize} />

<div class="flex h-screen overflow-hidden bg-yt-bg text-white">
  <!-- Left Sidebar -->
  <aside class="bg-yt-surface border-r border-slate-800 flex flex-col h-full shrink-0 transition-all duration-200 {collapsed ? 'w-16' : 'w-56'}">
    <div class="{collapsed ? 'p-2' : 'p-5'}">
      <!-- Logo -->
      <div class="flex items-center {collapsed ? 'justify-center mb-4' : 'gap-3 mb-6'}">
        <div class="w-8 h-8 rounded-lg bg-yt-primary flex items-center justify-center text-white shrink-0">
          <span class="material-symbols-outlined text-[20px]">download</span>
        </div>
        {#if !collapsed}
          <div>
            <h1 class="font-display font-bold text-base leading-tight">YT-DLP GUI</h1>
            <p class="text-slate-500 text-xs font-mono">v1.0.0</p>
          </div>
        {/if}
      </div>

      <!-- Navigation -->
      <nav class="flex flex-col gap-1">
        {#each navItems as item}
          <a
            href={item.href}
            class="flex items-center {collapsed ? 'justify-center px-2 py-2.5' : 'gap-3 px-3 py-2.5'} rounded-lg transition-colors relative
              {isActive(item.href) ? 'bg-yt-primary/20 text-yt-primary font-medium' : 'text-slate-400 hover:bg-yt-highlight hover:text-white'}"
            title={collapsed ? item.label : undefined}
          >
            <span class="material-symbols-outlined text-[20px]">{item.icon}</span>
            {#if !collapsed}
              <span class="text-sm">{item.label}</span>
            {/if}
            {#if item.icon === "download_done" && (activeCount + pendingCount) > 0}
              <span class="absolute {collapsed ? '-top-1 -right-1' : 'top-1 right-2'} w-5 h-5 bg-yt-primary text-white text-[10px] font-bold rounded-full flex items-center justify-center">
                {activeCount + pendingCount}
              </span>
            {/if}
          </a>
        {/each}
      </nav>
    </div>

    <!-- Sidebar bottom: toggle + drawer button -->
    <div class="mt-auto {collapsed ? 'p-2' : 'p-4'} space-y-2">
      <!-- Active Downloads Button -->
      <button
        onclick={() => drawerOpen = !drawerOpen}
        class="w-full flex items-center {collapsed ? 'justify-center px-2 py-2' : 'gap-2 px-3 py-2'} rounded-lg text-slate-400 hover:bg-yt-highlight hover:text-white transition-colors relative"
        title="Active Downloads"
      >
        <span class="material-symbols-outlined text-[20px]">downloading</span>
        {#if !collapsed}
          <span class="text-sm">Activity</span>
        {/if}
        {#if activeCount > 0}
          <span class="absolute {collapsed ? '-top-1 -right-1' : 'top-0.5 right-1'} w-4 h-4 bg-green-500 text-white text-[9px] font-bold rounded-full flex items-center justify-center animate-pulse">
            {activeCount}
          </span>
        {/if}
      </button>

      <!-- Collapse Toggle -->
      {#if !isSmall}
        <button
          onclick={() => sidebarCollapsed = !sidebarCollapsed}
          class="w-full flex items-center {collapsed ? 'justify-center px-2 py-2' : 'gap-2 px-3 py-2'} rounded-lg text-slate-500 hover:bg-yt-highlight hover:text-white transition-colors"
        >
          <span class="material-symbols-outlined text-[18px] transition-transform {collapsed ? 'rotate-180' : ''}">chevron_left</span>
          {#if !collapsed}
            <span class="text-xs">Collapse</span>
          {/if}
        </button>
      {/if}
    </div>
  </aside>

  <!-- Main Content -->
  <main class="flex-1 flex h-full overflow-hidden relative">
    <div class="absolute top-0 left-0 w-full h-[500px] bg-gradient-to-b from-yt-primary/5 to-transparent pointer-events-none z-0"></div>

    {#if checking}
      <div class="flex-1 flex items-center justify-center z-10">
        <div class="flex flex-col items-center gap-3">
          <span class="material-symbols-outlined text-yt-primary text-4xl animate-spin">progress_activity</span>
          <p class="text-sm text-slate-400">의존성 확인 중...</p>
        </div>
      </div>
    {:else if !depsInstalled}
      <div class="flex-1 flex flex-col items-center justify-center z-10 gap-6">
        <div class="w-16 h-16 rounded-xl bg-yt-primary/20 flex items-center justify-center">
          <span class="material-symbols-outlined text-yt-primary text-4xl">download</span>
        </div>
        <h2 class="font-display text-2xl font-bold">yt-dlp 설정 필요</h2>
        <p class="text-slate-400">YouTube 다운로드를 위해 yt-dlp를 설치해야 합니다.</p>

        {#if installError}
          <div class="bg-red-500/10 border border-red-500/20 rounded-xl px-6 py-3 text-red-400 text-sm max-w-md">
            {installError}
          </div>
        {/if}

        {#if installing}
          <div class="flex flex-col items-center gap-3">
            <span class="material-symbols-outlined text-yt-primary text-3xl animate-spin">progress_activity</span>
            <p class="text-sm text-slate-400">{installMessage}</p>
          </div>
        {:else}
          <button
            class="px-8 py-3 rounded-xl bg-yt-primary hover:bg-blue-600 text-white font-bold transition-all shadow-lg shadow-yt-primary/20"
            onclick={handleInstall}
          >
            yt-dlp 설치하기
          </button>
        {/if}
      </div>
    {:else}
      <div class="flex-1 z-10 overflow-hidden">
        {@render children()}
      </div>
    {/if}
  </main>

  <!-- Download Drawer Overlay -->
  {#if drawerOpen}
    <!-- Backdrop -->
    <button
      class="fixed inset-0 bg-black/40 z-40"
      onclick={() => drawerOpen = false}
      aria-label="Close drawer"
    ></button>

    <!-- Drawer -->
    <div class="fixed top-0 right-0 h-full w-80 bg-yt-surface border-l border-slate-800 z-50 flex flex-col shadow-2xl animate-slide-in">
      <!-- Header -->
      <div class="px-4 py-3 border-b border-slate-800 flex items-center justify-between shrink-0">
        <h3 class="font-display font-semibold text-sm">Active Downloads</h3>
        <button onclick={() => drawerOpen = false} class="text-slate-400 hover:text-white transition-colors p-1 rounded-lg hover:bg-yt-highlight">
          <span class="material-symbols-outlined text-[18px]">close</span>
        </button>
      </div>

      <!-- Active Downloads -->
      <div class="flex-1 overflow-y-auto hide-scrollbar">
        {#if activeDownloads.length === 0}
          <div class="flex flex-col items-center justify-center py-12 text-center px-4">
            <span class="material-symbols-outlined text-slate-600 text-4xl">cloud_done</span>
            <p class="text-slate-500 text-sm mt-2">활성 다운로드가 없습니다</p>
          </div>
        {:else}
          <div class="p-3 space-y-2">
            {#each activeDownloads as item}
              <div class="bg-yt-highlight rounded-lg p-3 border border-slate-800/50 {item.status === 'downloading' ? '!border-yt-primary/30' : ''}">
                <p class="text-sm text-white truncate font-medium">{item.title}</p>
                <div class="flex items-center justify-between mt-1.5">
                  {#if item.status === "downloading"}
                    <span class="text-xs text-yt-primary font-mono">{(item.progress || 0).toFixed(0)}%</span>
                    <span class="text-xs text-slate-400">{item.speed || "..."}</span>
                  {:else}
                    <span class="text-xs text-slate-500">Pending</span>
                  {/if}
                </div>
                {#if item.status === "downloading"}
                  <div class="w-full bg-slate-800 rounded-full h-1 mt-2">
                    <div class="bg-yt-primary h-1 rounded-full transition-all" style="width: {item.progress || 0}%"></div>
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        {/if}

        <!-- Recent Completed -->
        {#if recentCompleted.length > 0}
          <div class="border-t border-slate-800 mt-2">
            <div class="px-4 py-2">
              <h4 class="text-xs text-slate-500 font-medium uppercase tracking-wider">Recent</h4>
            </div>
            <div class="px-3 pb-3 space-y-1">
              {#each recentCompleted as item}
                <div class="flex items-center gap-2 px-2 py-1.5 rounded-lg">
                  <span class="material-symbols-outlined text-green-400 text-[16px]">check_circle</span>
                  <span class="text-sm text-slate-300 truncate">{item.title}</span>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  @keyframes slide-in {
    from { transform: translateX(100%); }
    to { transform: translateX(0); }
  }
  .animate-slide-in {
    animation: slide-in 0.2s ease-out;
  }
</style>

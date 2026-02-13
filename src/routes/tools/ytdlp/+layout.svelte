<script lang="ts">
  import { commands } from "$lib/bindings"
  import { Channel } from "@tauri-apps/api/core"
  import { page } from "$app/stores"
  import { onMount } from "svelte"

  let { children } = $props()

  let checking = $state(true)
  let depsInstalled = $state(false)
  let installing = $state(false)
  let installMessage = $state("")
  let installError = $state("")

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

  onMount(async () => {
    await checkDeps()
  })

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

<div class="flex h-screen overflow-hidden bg-yt-bg text-white">
  <!-- Left Sidebar -->
  <aside class="w-64 bg-yt-surface border-r border-slate-800 flex flex-col h-full shrink-0">
    <div class="p-6">
      <!-- Logo -->
      <div class="flex items-center gap-3 mb-8">
        <div class="w-8 h-8 rounded-lg bg-yt-primary flex items-center justify-center text-white">
          <span class="material-symbols-outlined text-[20px]">download</span>
        </div>
        <div>
          <h1 class="font-display font-bold text-lg leading-tight">YT-DLP GUI</h1>
          <p class="text-slate-500 text-xs font-mono">v1.0.0</p>
        </div>
      </div>

      <!-- Navigation -->
      <nav class="flex flex-col gap-2">
        {#each navItems as item}
          <a
            href={item.href}
            class="flex items-center gap-3 px-4 py-3 rounded-xl transition-colors
              {isActive(item.href) ? 'bg-yt-primary/20 text-yt-primary font-medium' : 'text-slate-400 hover:bg-yt-highlight hover:text-white'}"
          >
            <span class="material-symbols-outlined text-[22px]">{item.icon}</span>
            <span>{item.label}</span>
          </a>
        {/each}
      </nav>
    </div>
  </aside>

  <!-- Main Content -->
  <main class="flex-1 flex h-full overflow-hidden relative">
    <!-- Subtle gradient overlay -->
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
        <div class="w-16 h-16 rounded-2xl bg-yt-primary/20 flex items-center justify-center">
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
</div>

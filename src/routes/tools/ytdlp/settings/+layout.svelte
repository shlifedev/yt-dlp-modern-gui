<script lang="ts">
  import { page } from "$app/stores"
  import { t } from "$lib/i18n/index.svelte"

  let { children } = $props()

  const tabs = [
    { href: "/tools/ytdlp/settings", icon: "tune", labelKey: "settings.general", exact: true },
    { href: "/tools/ytdlp/settings/dependencies", icon: "extension", labelKey: "settings.dependencies", exact: false },
  ]

  function isActive(href: string, exact = false): boolean {
    const path = $page.url.pathname
    if (exact) return path === href
    return path.startsWith(href)
  }
</script>

<div class="flex-1 flex flex-col h-full bg-yt-bg">
  <header class="px-8 py-8 shrink-0 border-b border-yt-border bg-yt-surface/30">
    <h2 class="text-2xl font-bold text-yt-text tracking-tight">{t("settings.title")}</h2>
    <p class="text-sm text-yt-text-secondary mt-1">{t("settings.subtitle")}</p>
  </header>

  <div class="flex-1 flex min-h-0">
    <!-- Sub Sidebar -->
    <nav class="w-48 shrink-0 border-r border-yt-border bg-yt-surface/20 py-4 px-3 space-y-1 overflow-y-auto">
      {#each tabs as tab}
        <a
          href={tab.href}
          class="flex items-center gap-2.5 px-3 py-2 rounded-md transition-colors text-sm font-medium
            {isActive(tab.href, tab.exact)
              ? 'bg-yt-highlight text-yt-text ring-1 ring-inset ring-yt-border'
              : 'text-yt-text-secondary hover:bg-yt-overlay hover:text-yt-text'}"
        >
          <span class="material-symbols-outlined text-[18px] {isActive(tab.href, tab.exact) ? 'text-yt-primary' : ''}">{tab.icon}</span>
          <span>{t(tab.labelKey)}</span>
        </a>
      {/each}
    </nav>

    <!-- Settings Content -->
    <div class="flex-1 overflow-y-auto">
      {@render children()}
    </div>
  </div>
</div>

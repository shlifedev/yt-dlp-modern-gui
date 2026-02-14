<script lang="ts">
  import { commands } from "$lib/bindings"
  import { onMount } from "svelte"
  import { t, setLocale, getLocale, supportedLocales } from "$lib/i18n/index.svelte"
  import { setTheme, getTheme } from "$lib/theme/index.svelte"
  import { themes, themeList, type ThemeId } from "$lib/theme/themes"

  let settings = $state({
    downloadPath: "",
    defaultQuality: "1080p",
    maxConcurrent: 3,
    filenameTemplate: "%(title)s.%(ext)s",
    cookieBrowser: null as string | null,
    autoUpdateYtdlp: true,
    useAdvancedTemplate: false,
    templateUploaderFolder: false,
    templateUploadDate: false,
    templateVideoId: false,
    language: null as string | null,
    theme: null as string | null,
    minimizeToTray: null as boolean | null,
  })

  let browsers = $state<string[]>([])
  let loading = $state(true)
  let saving = $state(false)
  let saved = $state(false)
  let updateStatus = $state("")

  // 4-3: Separate try/catch for getSettings and getAvailableBrowsers
  onMount(async () => {
    try {
      const r = await commands.getSettings()
      if (r.status === "ok") settings = r.data
    } catch (e) { console.error("Failed to load settings:", e) }
    try {
      browsers = await commands.getAvailableBrowsers()
    } catch (e) { console.error("Failed to load browsers:", e) }
    loading = false
  })

  async function handleSave() {
    saving = true; saved = false
    try {
      const r = await commands.updateSettings(settings)
      if (r.status === "ok") { saved = true; setTimeout(() => saved = false, 2000) }
    } catch (e) { console.error(e) }
    finally { saving = false }
  }

  async function handleSelectDir() {
    try {
      const r = await commands.selectDownloadDirectory()
      if (r.status === "ok" && r.data) settings.downloadPath = r.data
    } catch (e) { console.error("Failed to select directory:", e) }
  }

  async function handleUpdateYtdlp() {
    updateStatus = t("settings.updating")
    try {
      const r = await commands.updateYtdlp()
      updateStatus = r.status === "ok" ? r.data : t("settings.updateFailed")
    } catch (e: any) { updateStatus = "실패: " + (e.message || e) }
  }

  async function handleLanguageChange(locale: string) {
    setLocale(locale)
    settings.language = locale
    // Auto-save so the change persists without requiring the user to click Save
    await handleSave()
  }

  async function handleThemeChange(themeId: string) {
    setTheme(themeId as ThemeId)
    settings.theme = themeId
    // Auto-save so the change persists without requiring the user to click Save
    await handleSave()
  }
</script>

<div class="flex-1 flex flex-col h-full bg-yt-bg">
  <header class="px-8 py-8 shrink-0 border-b border-yt-border bg-yt-surface/30">
    <h2 class="text-2xl font-bold text-yt-text tracking-tight">{t("settings.title")}</h2>
    <p class="text-sm text-yt-text-secondary mt-1">{t("settings.subtitle")}</p>
  </header>

  <div class="flex-1 overflow-y-auto">
    {#if loading}
      <div class="flex justify-center py-16">
        <span class="material-symbols-outlined text-yt-primary text-3xl animate-spin">progress_activity</span>
      </div>
    {:else}
      <div class="max-w-3xl mx-auto px-8 py-8 space-y-10">
        
        <!-- General Section -->
        <section>
          <h3 class="text-xs font-semibold text-yt-text-secondary uppercase tracking-wider mb-4 px-1">General</h3>
          <div class="bg-yt-surface border border-yt-border rounded-lg divide-y divide-yt-border/50 overflow-hidden">
             <!-- Download Path -->
             <div class="p-4 flex items-center justify-between gap-4">
                <div class="flex-1 min-w-0">
                   <label for="download-path" class="block text-sm font-medium text-yt-text mb-1">{t("settings.downloadPath")}</label>
                   <p class="text-xs text-yt-text-secondary truncate font-mono">{settings.downloadPath}</p>
                </div>
                <button 
                  class="px-3 py-1.5 rounded-md bg-yt-bg border border-yt-border text-yt-text text-xs font-medium hover:bg-yt-highlight transition-colors"
                  onclick={handleSelectDir}
                >
                  {t("settings.browse")}
                </button>
             </div>

             <!-- Concurrent Downloads -->
             <div class="p-4 flex items-center justify-between gap-4">
                <div>
                   <label for="concurrent-input" class="block text-sm font-medium text-yt-text mb-1">{t("settings.concurrent")}</label>
                   <p class="text-xs text-yt-text-secondary">Simultaneous downloads (1-10)</p>
                </div>
                <div class="flex items-center gap-3">
                   <input id="concurrent-input" type="range" class="w-32 accent-yt-primary" min="1" max="10" bind:value={settings.maxConcurrent} />
                   <span class="text-sm font-mono w-6 text-center text-yt-text font-bold">{settings.maxConcurrent}</span>
                </div>
             </div>
             
             <!-- Minimize to Tray -->
             <div class="p-4 flex items-center justify-between gap-4">
                <div>
                   <label for="minimize-tray" class="block text-sm font-medium text-yt-text mb-1">{t("settings.minimizeToTray")}</label>
                   <p class="text-xs text-yt-text-secondary">{t("settings.minimizeToTrayDesc")}</p>
                </div>
                <label class="relative inline-flex items-center cursor-pointer">
                  <input id="minimize-tray" type="checkbox" checked={settings.minimizeToTray === true} onchange={(e) => settings.minimizeToTray = (e.target as HTMLInputElement).checked} class="sr-only peer" />
                  <div class="w-9 h-5 bg-yt-border peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-yt-primary"></div>
                </label>
             </div>
          </div>
        </section>

        <!-- Network & Updates -->
        <section>
          <h3 class="text-xs font-semibold text-yt-text-secondary uppercase tracking-wider mb-4 px-1">Advanced</h3>
          <div class="bg-yt-surface border border-yt-border rounded-lg divide-y divide-yt-border/50 overflow-hidden">
             <!-- Cookie Browser -->
             <div class="p-4 flex items-center justify-between gap-4">
                <div>
                   <label for="cookie-browser" class="block text-sm font-medium text-yt-text mb-1">{t("settings.cookieBrowser")}</label>
                   <p class="text-xs text-yt-text-secondary">{t("settings.cookieHelp")}</p>
                </div>
                <select
                  id="cookie-browser"
                  class="bg-yt-bg text-yt-text border border-yt-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-yt-primary focus:outline-none"
                  bind:value={settings.cookieBrowser}
                >
                  <option value={null}>{t("settings.none")}</option>
                  {#each browsers as browser}
                    <option value={browser}>{browser}</option>
                  {/each}
                </select>
             </div>

             <!-- Auto Update -->
             <div class="p-4 flex items-center justify-between gap-4">
                <div>
                   <label for="auto-update" class="block text-sm font-medium text-yt-text mb-1">{t("settings.autoUpdate")}</label>
                   <p class="text-xs text-yt-text-secondary">{t("settings.autoUpdateDesc")}</p>
                </div>
                <div class="flex items-center gap-3">
                   {#if updateStatus}
                      <span class="text-xs text-yt-text-secondary">{updateStatus}</span>
                   {/if}
                   <button 
                      class="text-yt-primary hover:underline text-xs"
                      onclick={handleUpdateYtdlp}
                   >
                      {t("settings.updateNow")}
                   </button>
                   <label class="relative inline-flex items-center cursor-pointer ml-2">
                     <input id="auto-update" type="checkbox" bind:checked={settings.autoUpdateYtdlp} class="sr-only peer" />
                     <div class="w-9 h-5 bg-yt-border peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-yt-primary"></div>
                   </label>
                </div>
             </div>
          </div>
        </section>

        <!-- Appearance -->
        <section>
          <h3 class="text-xs font-semibold text-yt-text-secondary uppercase tracking-wider mb-4 px-1">Appearance</h3>
          <div class="bg-yt-surface border border-yt-border rounded-lg divide-y divide-yt-border/50 overflow-hidden">
             <!-- Language -->
             <div class="p-4 flex items-center justify-between gap-4">
                <label for="language-select" class="block text-sm font-medium text-yt-text">{t("settings.language")}</label>
                <select
                  id="language-select"
                  class="bg-yt-bg text-yt-text border border-yt-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-yt-primary focus:outline-none"
                  value={getLocale()}
                  onchange={(e) => handleLanguageChange((e.target as HTMLSelectElement).value)}
                >
                  {#each supportedLocales as loc}
                    <option value={loc.code}>{loc.name}</option>
                  {/each}
                </select>
             </div>

             <!-- Theme -->
             <div class="p-4">
                <h4 class="block text-sm font-medium text-yt-text mb-3">{t("settings.theme")}</h4>
                <div class="grid grid-cols-4 gap-3">
                  {#each themeList as themeItem}
                    <button
                      class="flex flex-col items-center gap-2 p-3 rounded-lg border transition-all {getTheme() === themeItem.id ? 'border-yt-primary bg-yt-primary/5 ring-1 ring-yt-primary' : 'border-yt-border hover:bg-yt-highlight'}"
                      onclick={() => handleThemeChange(themeItem.id)}
                    >
                      <div class="flex gap-1">
                        <div class="w-3 h-3 rounded-full border border-black/10" style="background-color: {themes[themeItem.id].primary}"></div>
                        <div class="w-3 h-3 rounded-full border border-black/10" style="background-color: {themes[themeItem.id].bg}"></div>
                        <div class="w-3 h-3 rounded-full border border-black/10" style="background-color: {themes[themeItem.id].surface}"></div>
                      </div>
                      <span class="text-[10px] text-yt-text font-medium">{t(themeItem.labelKey)}</span>
                    </button>
                  {/each}
                </div>
             </div>
          </div>
        </section>

        <!-- Save Action -->
        <div class="pt-4 flex justify-end">
           <button
            class="px-6 py-2.5 rounded-lg bg-yt-primary hover:bg-yt-primary-hover text-white text-sm font-semibold shadow-sm transition-all flex items-center gap-2 disabled:opacity-50"
            onclick={handleSave}
            disabled={saving}
          >
            {#if saving}
              <span class="material-symbols-outlined text-[18px] animate-spin">progress_activity</span>
              <span>{t("settings.saving")}</span>
            {:else if saved}
              <span class="material-symbols-outlined text-[18px]">check</span>
              <span>{t("settings.saved")}</span>
            {:else}
              <span>{t("settings.save")}</span>
            {/if}
          </button>
        </div>
      </div>
    {/if}
  </div>
</div>

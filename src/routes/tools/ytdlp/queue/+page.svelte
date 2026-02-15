<script lang="ts">
  import { commands } from "$lib/bindings"
  import { onMount, onDestroy } from "svelte"
  import { t } from "$lib/i18n/index.svelte"
  import { slide, fade } from "svelte/transition"
  import { flip } from "svelte/animate"
  import { formatSize } from "$lib/utils/format"

  let queue = $state<any[]>([])
  let firstLoad = $state(true)
  let expandedErrors = $state<Set<number>>(new Set())

  // 5-4: Consolidated single onMount with interval after initial load
  let interval: ReturnType<typeof setInterval>
  onMount(async () => {
    await loadQueue()
    firstLoad = false
    interval = setInterval(loadQueue, 2000)
  })

  onDestroy(() => { if (interval) clearInterval(interval) })

  async function loadQueue() {
    try {
      const result = await commands.getDownloadQueue()
      if (result.status === "ok") {
        queue = result.data
      }
    } catch (e) {
      console.error("Failed to load queue:", e)
    }
  }

  // 4-1: Add try/catch to prevent unhandled errors
  async function handleClearCompleted() {
    try {
      const result = await commands.clearCompleted()
      if (result.status === "ok") await loadQueue()
    } catch (e) {
      console.error("Failed to clear completed:", e)
    }
  }

  async function handleCancel(id: number) {
    try {
      await commands.cancelDownload(id)
      await loadQueue()
    } catch (e) {
      console.error("Failed to cancel download:", e)
    }
  }

  let activeCount = $derived(queue.filter(q => q.status === "downloading").length)
  let pendingCount = $derived(queue.filter(q => q.status === "pending").length)
  let completedCount = $derived(queue.filter(q => q.status === "completed").length)

  async function handleCancelAll() {
    try {
      const result = await commands.cancelAllDownloads()
      if (result.status === "ok") await loadQueue()
    } catch (e) {
      console.error("Failed to cancel all downloads:", e)
    }
  }

  function toggleError(id: number) {
    const next = new Set(expandedErrors)
    if (next.has(id)) next.delete(id)
    else next.add(id)
    expandedErrors = next
  }

</script>

<div class="flex-1 flex flex-col h-full bg-yt-bg">
  <header class="px-6 py-6 shrink-0 border-b border-yt-border bg-yt-surface/30">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-lg font-semibold text-yt-text">{t("queue.title")}</h2>
        <p class="text-xs text-yt-text-secondary mt-1">{t("queue.subtitle")}</p>
      </div>
      <div class="flex gap-2">
        <button
          class="px-3 py-1.5 rounded-md bg-yt-warning/10 text-yt-warning hover:bg-yt-warning/20 text-xs font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          onclick={handleCancelAll}
          disabled={activeCount + pendingCount === 0}
        >
          {t("queue.cancelAll")}
        </button>
        <button
          class="px-3 py-1.5 rounded-md bg-yt-surface hover:bg-yt-highlight border border-yt-border text-yt-text text-xs font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          onclick={handleClearCompleted}
          disabled={completedCount === 0}
        >
          {t("queue.clearCompleted")}
        </button>
      </div>
    </div>

    <!-- Stats -->
    <div class="flex gap-6 mt-4">
      <div class="flex items-center gap-2">
        <span class="w-2 h-2 rounded-full bg-yt-primary"></span>
        <span class="text-xs font-medium text-yt-text">{activeCount}</span>
        <span class="text-xs text-yt-text-secondary">{t("queue.active")}</span>
      </div>
      <div class="flex items-center gap-2">
         <span class="w-2 h-2 rounded-full bg-yt-success"></span>
        <span class="text-xs font-medium text-yt-text">{completedCount}</span>
        <span class="text-xs text-yt-text-secondary">{t("queue.completed")}</span>
      </div>
      <div class="flex items-center gap-2">
         <span class="w-2 h-2 rounded-full bg-yt-text-muted"></span>
        <span class="text-xs font-medium text-yt-text">{queue.length}</span>
        <span class="text-xs text-yt-text-secondary">{t("queue.total")}</span>
      </div>
    </div>
  </header>

  <div class="flex-1 overflow-y-auto">
    {#if firstLoad}
      <div class="flex justify-center py-16">
        <span class="material-symbols-outlined text-yt-primary text-3xl animate-spin">progress_activity</span>
      </div>
    {:else if queue.length === 0}
      <div class="flex flex-col items-center justify-center h-64 text-center" in:fade>
        <span class="material-symbols-outlined text-yt-border text-5xl mb-2 animate-float">inbox</span>
        <p class="text-yt-text-secondary text-sm">{t("queue.emptyDesc")}</p>
      </div>
    {:else}
      <div class="divide-y divide-yt-border/50">
        {#each queue as item (item.id)}
          <div 
            class="group flex items-center gap-4 px-6 py-3 hover:bg-yt-highlight/30 transition-all duration-200 hover:scale-[1.002] active:scale-[0.998]"
            transition:slide|local={{ duration: 200 }}
            animate:flip={{ duration: 300 }}
          >
            <!-- Icon/Status -->
            <div class="shrink-0">
               {#if item.status === "downloading"}
                  <div class="w-8 h-8 rounded-full bg-yt-primary/10 flex items-center justify-center">
                     <span class="material-symbols-outlined text-yt-primary text-[18px] animate-spin">progress_activity</span>
                  </div>
               {:else if item.status === "completed"}
                  <div class="w-8 h-8 rounded-full bg-yt-success/10 flex items-center justify-center">
                     <span class="material-symbols-outlined text-yt-success text-[18px]">check</span>
                  </div>
               {:else if item.status === "failed"}
                   <div class="w-8 h-8 rounded-full bg-yt-error/10 flex items-center justify-center">
                     <span class="material-symbols-outlined text-yt-error text-[18px]">error</span>
                  </div>
               {:else}
                  <div class="w-8 h-8 rounded-full bg-yt-surface border border-yt-border flex items-center justify-center">
                     <span class="material-symbols-outlined text-yt-text-muted text-[18px]">hourglass_empty</span>
                  </div>
               {/if}
            </div>

            <!-- Info -->
            <div class="flex-1 min-w-0">
               <div class="flex items-center justify-between mb-1">
                  <h4 class="font-medium text-yt-text text-sm truncate pr-4">{item.title}</h4>
                  <span class="text-[10px] px-1.5 py-0.5 rounded bg-yt-surface border border-yt-border text-yt-text-secondary whitespace-nowrap">{item.qualityLabel || "N/A"}</span>
               </div>
               
               <div class="flex items-center justify-between">
                  <div class="flex items-center gap-3 text-xs text-yt-text-secondary">
                     {#if item.status === "downloading"}
                        <span class="text-yt-primary font-mono">{item.speed || "0 KiB/s"}</span>
                        <span class="text-yt-text-muted">ETA: {item.eta || "--:--"}</span>
                     {:else if item.status === "completed"}
                        <span class="text-yt-success">{t("queue.downloadComplete")}</span>
                     {:else if item.status === "failed"}
                         <button class="text-yt-error hover:underline flex items-center gap-1" onclick={() => toggleError(item.id)}>
                            {item.errorMessage ? item.errorMessage.split("\n")[0] : t("queue.failed")}
                            <span class="material-symbols-outlined text-[14px]">expand_more</span>
                         </button>
                     {:else}
                        <span>{t("queue.pendingStatus")}</span>
                     {/if}
                  </div>
                  
                  {#if item.status === "downloading"}
                    <div class="w-32 bg-yt-surface rounded-full h-1.5 border border-yt-border/50 overflow-hidden relative">
                       <div class="bg-yt-primary h-full transition-all duration-300 relative overflow-hidden" style="width: {item.progress || 0}%">
                          <div class="absolute inset-0 animate-shimmer"></div>
                       </div>
                    </div>
                  {/if}
               </div>
               
               {#if item.status === "failed" && item.errorMessage && expandedErrors.has(item.id)}
                 <div class="mt-2 text-xs text-yt-error bg-yt-error/5 p-2 rounded border border-yt-error/10 font-mono whitespace-pre-wrap">
                    {item.errorMessage}
                 </div>
               {/if}
            </div>

            <!-- Actions -->
            <div class="shrink-0 pl-2">
               {#if item.status === "downloading" || item.status === "pending"}
                 <button 
                  class="p-1.5 rounded-md hover:bg-yt-error/10 text-yt-text-muted hover:text-yt-error transition-colors"
                  onclick={() => handleCancel(item.id)}
                  title="Cancel"
                 >
                    <span class="material-symbols-outlined text-[18px]">close</span>
                 </button>
               {/if}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

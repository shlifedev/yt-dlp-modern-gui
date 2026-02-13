<script lang="ts">
  import { goto } from "$app/navigation"
  import { onMount } from "svelte"

  onMount(() => {
    goto("/tools/ytdlp")
  })
</script>

<div class="flex items-center justify-center h-screen bg-yt-bg">
  <span class="material-symbols-outlined text-yt-primary text-4xl animate-spin">progress_activity</span>
</div>
